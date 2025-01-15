#![allow(dead_code)]
use rayon::prelude::*;
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct ChunkList<T> {
    my_list: Vec<Vec<T>>,
    chunk_size: usize,
}

impl<T> Default for ChunkList<T>
where T: Ord + Debug + Send + Sync + Clone, {
    /// Default constructor with chunk size = 1000
    fn default() -> Self {
        Self::new(1000)
    }
}

impl<T> ChunkList<T>
where T: Ord + Debug +Send + Sync + Clone, {
    /// Creates a new ChunkList with the specified chunk size.
    pub fn new(chunk_size: usize) -> Self {
        Self {
            my_list: Vec::new(),
            chunk_size,
        }
    }

    /// Helper: Convert a global index to (chunk_index, position_in_chunk).
    fn index_to_chunk_pos(&self, index: usize) -> (usize, usize) {
        let chunk_index = index / self.chunk_size;
        let pos = index % self.chunk_size;
        (chunk_index, pos)
    }

    /// Add an element to the list, finding a chunk that isn't full or creating a new one.
    pub fn add(&mut self, t: T) {
        for chunk in &mut self.my_list {
            if chunk.len() < self.chunk_size {
                chunk.push(t);
                return;
            }
        }
        // If we get here, all chunks are full -> create a new chunk
        let mut new_chunk = Vec::with_capacity(self.chunk_size);
        new_chunk.push(t);
        self.my_list.push(new_chunk);
    }

    /// Add with optional rebalance: chooses between 5% of total size or sqrt(total size).
    pub fn add_optimized(&mut self, t: T, optimize_sqrt_size: bool) {
        self.add(t);
        self.set_chunk_size_optimized(optimize_sqrt_size);
    }

    /// Remove the first occurrence of `t`, in parallel.
    /// If found, short-circuits further removals using an AtomicBool.
    pub fn remove(&mut self, t: &T) {
        let found = AtomicBool::new(false);
        // We need parallel mutation over multiple chunks, so we do par_iter_mut.
        // Each chunk is independent, so this is safe so long as we only remove from one chunk.
        self.my_list.par_iter_mut().for_each(|chunk| {
            if found.load(Ordering::Relaxed) {
                // Another thread removed the item already
                return;
            }
            // Attempt binary search in this chunk
            if let Ok(idx) = chunk.binary_search(t) {
                // CAS to become the thread that does the removal
                let was_found = found.swap(true, Ordering::Relaxed);
                if !was_found {
                    // We are the first to swap from false -> true
                    chunk.remove(idx);
                }
            }
        });
    }

    /// Remove all instances of `t`, in parallel (each chunk will remove all matches).
    pub fn remove_all(&mut self, t: &T) {
        // We can do chunk.retain(...). We'll do it in parallel:
        self.my_list.par_iter_mut().for_each(|chunk| {
            chunk.retain(|x| x != t);
        });
    }

    /// Remove all + optional rebalance
    pub fn remove_all_optimized(&mut self, t: &T, optimize_sqrt_size: bool) {
        self.remove_all(t);
        self.set_chunk_size_optimized(optimize_sqrt_size);
    }

    /// Remove element at global index; recursively tries index+1 if out of range.
    pub fn remove_at(&mut self, index: usize) {
        // if index >= self.len() {
        //     panic!("Index out of range");
        // }
        // let (chunk_index, pos) = self.index_to_chunk_pos(index);
        // // Might fail if chunk indexing is off by 1 in some edge cases.
        // // The original C# code calls removeAt(index+1) on ArgOutOfRange; we’ll just panic here for clarity.
        // self.my_list[chunk_index].remove(pos);

        if index >= self.len() {
            panic!("Index out of range");
        }
        
        let (chunk_index, pos) = self.index_to_chunk_pos(index);

        // Check if chunk_index is valid
        if chunk_index >= self.my_list.len() {
            // Shift + 1
            self.remove_at(index + 1);
            return;
        }

        let chunk = &mut self.my_list[chunk_index];

        // Check if pos is valid within this chunk
        if pos >= chunk.len() {
            // Shift + 1
            self.remove_at(index + 1);
            return;
        }

        // If we get here, remove the element
        chunk.remove(pos);
    }

    /// Set an item at a particular index.
    pub fn set(&mut self, index: usize, t: T) {
        if index >= self.len() {
            panic!("Index out of range");
        }
        let (chunk_index, pos) = self.index_to_chunk_pos(index);
        self.my_list[chunk_index][pos] = t;
    }

    /// Get an item at a particular index.
    pub fn get(&self, index: usize) -> &T {
        // if index >= self.len() {
        //     panic!("Index out of range");
        // }
        // let (chunk_index, pos) = self.index_to_chunk_pos(index);
        // &self.my_list[chunk_index][pos]
        // If the user requested an index beyond the total size, 
        // we bail out (like the C# code eventually throws).
        
        if index >= self.len() {
            panic!("Index out of range");
        }
        
        let (chunk_index, pos) = self.index_to_chunk_pos(index);

        // If the chunk_index is out of range, shift + 1.
        if chunk_index >= self.my_list.len() {
            // Try again with index+1
            return self.get(index + 1);
        }
        
        let chunk = &self.my_list[chunk_index];
        
        // If pos is out of range for this chunk, shift + 1.
        if pos >= chunk.len() {
            // Try again with index+1
            return self.get(index + 1);
        }
        
        // Otherwise, we got it
        &chunk[pos]
    }

    /// Return a new Vec containing all elements from all chunks (in order).
    pub fn get_list(&self) -> Vec<T> {
        // We'll just flatten them in sequence
        let mut items = Vec::with_capacity(self.len());
        for chunk in &self.my_list {
            items.extend(chunk.iter().cloned());
        }
        items
    }

    /// Check if the list contains a given item, in parallel.
    pub fn contains(&self, t: &T) -> bool {
        // We can do a nested parallel approach:
        self.my_list
            .par_iter()
            .any(|chunk| chunk.par_iter().any(|item| item == t))
    }

    /// Clear the entire list (remove all chunks).
    pub fn clear(&mut self) {
        self.my_list.clear();
    }

    /// Return the total number of elements (sum of chunk lengths).
    pub fn len(&self) -> usize {
        self.my_list.iter().map(|v| v.len()).sum()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set a new chunk size and rebalance the elements.
    pub fn set_chunk_size(&mut self, new_chunk_size: usize) {
        if new_chunk_size > self.chunk_size {
            // If bigger, we can just set the chunk size. 
            // The C# code rebalances only if we are shrinking.
            self.chunk_size = new_chunk_size;
        } else {
            // Rebalance all
            let items = self.get_list();
            self.clear();
            self.chunk_size = new_chunk_size;
            for item in items {
                self.add(item);
            }
        }
    }

    /// Optimized version of `set_chunk_size`: choose between 5% or sqrt of total size.
    pub fn set_chunk_size_optimized(&mut self, optimize_sqrt_size: bool) {
        let total_size = self.len();
        if total_size == 0 {
            return;
        }
        let new_size = if optimize_sqrt_size {
            // use sqrt(total_size)
            (total_size as f64).sqrt() as usize
        } else {
            // use 5% of total size
            (total_size as f64 * 0.05).round() as usize
        };
        if new_size == 0 {
            // fallback to 1 if 5% is 0
            self.set_chunk_size(1);
        } else {
            self.set_chunk_size(new_size);
        }
    }

    /// Get current chunk size
    pub fn get_chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Get amount of chunks in the list
    pub fn chunk_amount(&self) -> usize {
        self.my_list.len()
    }

    /// Sort the entire list. We gather everything, sort in parallel, then rebuild.
    pub fn sort(&mut self) {
        let mut items = self.get_list();
        // Parallel sort from Rayon
        items.par_sort();
        self.clear();
        for item in items {
            self.add(item);
        }
    }

    /// Print all items, chunk by chunk (for debugging).
    pub fn print(&self) {
        for (i, chunk) in self.my_list.iter().enumerate() {
            println!("Chunk #{}", i + 1);
            for item in chunk {
                print!("{:?} ", item);
            }
            println!();
        }
    }
}
