// Run with "cargo test -- --nocapture" to see println output
use rand::Rng;
use std::time::Instant;
use chunklist::ChunkList;

#[test]
fn vector_comparison() {
    println!("Starting");

    // We'll compare a plain Vec<i32> to two ChunkLists with different chunk sizes
    let mut list2: Vec<i32> = Vec::new();
    // Large chunk size
    let mut list = ChunkList::new(50_000);
    // sqrt chunk size
    let mut list3 = ChunkList::new((500_000f64).sqrt() as usize);

    let mut rng = rand::thread_rng();

    // Populate each with 500k random integers in [0..10).
    for _ in 0..500_000 {
        let x = rng.gen_range(0..10);
        list.add(x);
        list2.push(x);
        list3.add(x);
    }

    // region: ChunkListTest
    let start = Instant::now();

    println!(
        "\n\n{}\n{}\n{}\n\nSize: {}\n\n",
        list.contains(&3),
        list.contains(&6),
        list.contains(&500),
        list.len()
    );
    let elapsed = start.elapsed();
    println!(
        "[Elapsed time - Chunk List] {} min, {}.{} sec",
        elapsed.as_secs() / 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis()
    );

    println!("Sorting & Removing:\n\n");
    list.sort();
    list.remove(&7);
    list.remove_all(&3);

    let total_elapsed = start.elapsed();
    println!(
        "Completely Done\n[Elapsed time - Chunk List] {} min, {}.{} sec",
        total_elapsed.as_secs() / 60,
        total_elapsed.as_secs() % 60,
        total_elapsed.subsec_millis()
    );
    // endregion

    println!("\n******************************************************************");

    // region: ChunkListSqrtTest
    let start3 = Instant::now();
    println!(
        "\n\n{}\n{}\n{}\n\nSize: {}\n\n",
        list3.contains(&3),
        list3.contains(&6),
        list3.contains(&500),
        list3.len()
    );
    let elapsed3 = start3.elapsed();
    println!(
        "[Elapsed time - Chunk List Sqrt] {} min, {}.{} sec",
        elapsed3.as_secs() / 60,
        elapsed3.as_secs() % 60,
        elapsed3.subsec_millis()
    );

    println!("Sorting & Removing:\n\n");
    list3.sort();
    list3.remove(&7);
    list3.remove_all(&3);

    let total_elapsed3 = start3.elapsed();
    println!(
        "Completely Done\n[Elapsed time - Chunk List Sqrt] {} min, {}.{} sec",
        total_elapsed3.as_secs() / 60,
        total_elapsed3.as_secs() % 60,
        total_elapsed3.subsec_millis()
    );
    // endregion

    println!("\n******************************************************************\n");

    // region: ArrayListTest
    let start2 = Instant::now();
    println!(
        "\n\n{}\n{}\n{}\n\nSize: {}\n\n",
        list2.contains(&3),
        list2.contains(&6),
        list2.contains(&500),
        list2.len()
    );
    let elapsed2 = start2.elapsed();
    println!(
        "[Elapsed time - Vector] {} min, {}.{} sec",
        elapsed2.as_secs() / 60,
        elapsed2.as_secs() % 60,
        elapsed2.subsec_millis()
    );

    println!("Sorting & Removing:\n\n");
    list2.sort();
    // remove first occurrence of 7
    if let Some(pos) = list2.iter().position(|&y| y == 7) {
        list2.remove(pos);
    }
    // remove all 3
    let mut i = 0;
    while i < list2.len() {
        if list2[i] == 3 {
            list2.remove(i);
        } else {
            i += 1;
        }
    }

    let total_elapsed2 = start2.elapsed();
    println!(
        "Completely Done\n[Elapsed time - Vector] {} min, {}.{} sec",
        total_elapsed2.as_secs() / 60,
        total_elapsed2.as_secs() % 60,
        total_elapsed2.subsec_millis()
    );
    // endregion

    println!("\n******************************************************************\n");

    // convert times to float seconds for simpler comparison
    let time1 = total_elapsed.as_secs_f64();
    let time2 = total_elapsed2.as_secs_f64();
    let time3 = total_elapsed3.as_secs_f64();

    // The original test checks that time1 < time2
    assert!(time1 < time2, "Expected chunk list to be faster than vector");

    if time1 < time2 {
        println!(
            "Chunk List ran {:.2}x faster than Vector",
            time2 / time1
        );
    } else {
        println!(
            "Chunk List ran {:.2}x slower than Vector",
            time1 / time2
        );
    }

    if time3 < time2 {
        println!(
            "Sqrt Chunk List ran {:.2}x faster than Vector",
            time2 / time3
        );
    } else {
        println!(
            "Sqrt Chunk List ran {:.2}x slower than Vector",
            time3 / time2
        );
    }

    if time1 < time3 {
        println!(
            "Chunk List ran {:.2}x faster than Sqrt Chunk List\n\n",
            time3 / time1
        );
    } else {
        println!(
            "Chunk List ran {:.2}x slower than Sqrt Chunk List\n\n",
            time1 / time3
        );
    }
}

#[test]
fn chunk_size_comparison_macro() {
    // Mirror the chunk sizes tested in C#
    let mut list1 = ChunkList::new(100);
    let mut list2 = ChunkList::new(500);
    let mut list3 = ChunkList::new(1_000);
    let mut list4 = ChunkList::new(2_500);
    let mut list5 = ChunkList::new(5_000);
    let mut list6 = ChunkList::new(10_000);
    let mut list7 = ChunkList::new(25_000);
    let mut list8 = ChunkList::new(50_000);
    let mut list9 = ChunkList::new(100_000);
    let mut list10 = ChunkList::new(500_000);
    let mut list11 = ChunkList::new((500_000f64).sqrt() as usize);

    // (list0 with chunk size 10 is omitted for speed reasons)
    let mut rng = rand::thread_rng();

    // Fill them with 500k random values
    for _ in 0..500_000 {
        let x = rng.gen_range(0..10);
        list1.add(x);
        list2.add(x);
        list3.add(x);
        list4.add(x);
        list5.add(x);
        list6.add(x);
        list7.add(x);
        list8.add(x);
        list9.add(x);
        list10.add(x);
        list11.add(x);
    }

    fn test_list(list: &mut ChunkList<i32>) -> std::time::Duration {
        println!(
            "Testing list of 500,000 items with chunk size of {}",
            list.get_chunk_size()
        );
        let start = Instant::now();

        // println!(
        //     "\n\n{}\n{}\n{}\n\nSize: {}\n\n",
        //     list.contains(&3),
        //     list.contains(&6),
        //     list.contains(&500),
        //     list.size()
        // );

        list.sort();
        list.remove(&7);
        list.remove_all(&3);

        start.elapsed()
    }

    let results = vec![
        (list1.get_chunk_size(), test_list(&mut list1)),
        (list2.get_chunk_size(), test_list(&mut list2)),
        (list3.get_chunk_size(), test_list(&mut list3)),
        (list4.get_chunk_size(), test_list(&mut list4)),
        (list5.get_chunk_size(), test_list(&mut list5)),
        (list6.get_chunk_size(), test_list(&mut list6)),
        (list7.get_chunk_size(), test_list(&mut list7)),
        (list8.get_chunk_size(), test_list(&mut list8)),
        (list9.get_chunk_size(), test_list(&mut list9)),
        (list10.get_chunk_size(), test_list(&mut list10)),
        (list11.get_chunk_size(), test_list(&mut list11)),
    ];

    for (sz, duration) in results {
        println!(
            "Result for chunk size of {}: {} min, {}.{} sec",
            sz,
            duration.as_secs() / 60,
            duration.as_secs() % 60,
            duration.subsec_millis()
        );
    }

    println!(
        "Chunk size of {} is the square-root of 500000",
        (500_000f64).sqrt() as usize
    );
}

#[test]
fn chunk_size_comparison_micro() {
    // We'll replicate the logic: generate many ChunkLists with random sampleSizes,
    // half use 5% chunk size, half use sqrt chunk size, and compare times.

    use std::cmp::Ordering;

    struct MicroResult {
        duration: std::time::Duration,
        sample_size: usize,
    }

    let mut rng = rand::thread_rng();

    let mut big_list_percent = Vec::new();
    let mut big_list_sqrt = Vec::new();
    let mut sample_sizes = Vec::new();

    // Fill up 30 pairs
    for _ in 0..30 {
        let sample_size = rng.gen_range(100..10_000);
        let chunk_size_5p = ((sample_size as f64) * 0.05) as usize;
        let chunk_size_sqrt = (sample_size as f64).sqrt() as usize;

        let mut list5p = ChunkList::new(chunk_size_5p.max(1));
        let mut list_sqrt = ChunkList::new(chunk_size_sqrt.max(1));

        // Fill each list
        for _j in 0..sample_size {
            let val = rng.gen_range(0..10);
            list5p.add(val);
            list_sqrt.add(val);
        }

        big_list_percent.push(list5p);
        big_list_sqrt.push(list_sqrt);
        sample_sizes.push(sample_size);
    }

    fn test_list(list: &mut ChunkList<i32>) -> std::time::Duration {
        let start = Instant::now();

        // println!(
        //     "\n\n{}\n{}\n{}\n\nSize: {}\n\n",
        //     list.contains(&3),
        //     list.contains(&6),
        //     list.contains(&500),
        //     list.size()
        // );

        list.sort();
        list.remove(&7);
        list.remove_all(&3);

        start.elapsed()
    }

    let mut result_list_percent = Vec::new();
    let mut result_list_sqrt = Vec::new();

    // test the 5% chunk lists
    for (i, list) in big_list_percent.iter_mut().enumerate() {
        let dur = test_list(list);
        result_list_percent.push(MicroResult {
            duration: dur,
            sample_size: sample_sizes[i],
        });
    }

    // test the sqrt chunk lists
    for (i, list) in big_list_sqrt.iter_mut().enumerate() {
        let dur = test_list(list);
        result_list_sqrt.push(MicroResult {
            duration: dur,
            sample_size: sample_sizes[i],
        });
    }

    println!();
    for result in &result_list_percent {
        println!(
            "Result for (chunk size of ~5% of) list size {}: {} min, {}.{} sec",
            result.sample_size,
            result.duration.as_secs() / 60,
            result.duration.as_secs() % 60,
            result.duration.subsec_millis()
        );
    }

    for result in &result_list_sqrt {
        println!(
            "Result for (chunk size of sqrt of) list size {}: {} min, {}.{} sec",
            result.sample_size,
            result.duration.as_secs() / 60,
            result.duration.as_secs() % 60,
            result.duration.subsec_millis()
        );
    }

    // Count how many times 5% was faster vs sqrt
    let mut fin_res_percent = 0;
    let mut fin_res_sqrt = 0;

    for i in 0..result_list_percent.len() {
        match result_list_percent[i].duration.cmp(&result_list_sqrt[i].duration) {
            Ordering::Less => fin_res_percent += 1,
            Ordering::Greater => fin_res_sqrt += 1,
            Ordering::Equal => {
                // tie, do nothing
            }
        }
    }

    println!(
        "\nOut of 30 events, chunk size of 5% the sample size was faster {} times",
        fin_res_percent
    );
    println!(
        "Out of 30 events, chunk size of sqrt the sample size was faster {} times",
        fin_res_sqrt
    );
}
