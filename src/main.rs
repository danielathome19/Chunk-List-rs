use rand::Rng;
use chunklist::ChunkList;

fn main() {
    println!("Hello, world!");
    let mut chunklist = ChunkList::new(25);
    let mut rng = rand::thread_rng();
    for _ in 0..250 {
        let value = rng.gen_range(0..1000);
        chunklist.add(value);
    }
    chunklist.sort();
    chunklist.print();
    println!("Length: {}", chunklist.len());
    println!("List contains 500: {}\n", chunklist.contains(&500));
    
    let mut chunklist2 = ChunkList::<String>::new(20);
    // chunklist2.add("Hello".to_string());
    for _ in 0..100 {
        let mut rnd_string = String::new();
        for _j in 0..rand::thread_rng().gen_range(1..=10) {
            let rnd_char = char::from_u32(rand::thread_rng().gen_range(48..=126)).unwrap();
            rnd_string.push(rnd_char);
        }
        chunklist2.add(rnd_string);
    }
    chunklist2.print();

}
