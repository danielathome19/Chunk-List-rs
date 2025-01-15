[![Crate](https://img.shields.io/crates/v/chunklist.svg)](https://crates.io/crates/chunklist)
[![CI/CT/CD](https://github.com/danielathome19/Chunk-List-rs/actions/workflows/cargo_push.yml/badge.svg)](https://github.com/danielathome19/Chunk-List-rs/actions/workflows/cargo_push.yml)
[![License](https://img.shields.io/github/license/danielathome19/Chunk-List-rs.svg)](https://github.com/danielathome19/Chunk-List-rs/blob/main/LICENSE.md)
[![DOI](https://zenodo.org/badge/DOI/10.48550/arxiv.2101.00172.svg)](https://doi.org/10.48550/arxiv.2101.00172)

# About
A Chunk List is a new, concurrent, chunk-based data structure that is easily modifiable and allows for fast runtime operations.

To find out more, check out the provided research paper in the [original C# implementation repo](https://github.com/danielathome19/Chunk-List):
  * /Chunk List/Presentation/"Chunk List.pdf" (DOI: [10.48550/arxiv.2101.00172](https://doi.org/10.48550/arxiv.2101.00172))

## Installation
Install the [`chunklist` Cargo crate](https://crates.io/crates/chunklist):

```bash
cargo add chunklist
```

Or, in Cargo.toml:
```yml
[dependencies]
chunklist = "0.1.0"
```



# Usage
```rs
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
}
```

The __Presentation__ folder (i.e., the research paper) in the [original C# implementation repo](https://github.com/danielathome19/Chunk-List) contains a full presentation and research paper in PDF format, containing the following information:
  * What is a chunk list?
  * Where is a chunk list used?
  * Implementation details (construction, basic methods)
  * Complexity Analysis (Big-O)
  * Unit Testing
  * Integration

Program files are kept within the _main_ branch.

A full implementation of the class is kept within the __chunklist.rs__ file in the module __chunklist__, to be included within the program.

The __tests/chunklist_tests.rs__ file contains a benchmark test for comparison between a Vector and Chunk List.

# Bugs/Features
Bugs are tracked using the GitHub Issue Tracker.

Please use the issue tracker for the following purpose:
  * To raise a bug request; do include specific details and label it appropriately.
  * To suggest any improvements in existing features.
  * To suggest new features or structures or applications.

# License
The code is licensed under Apache License 2.0.

# Citation
If you use this code for your research, please cite this project:
```bibtex
@software{Szelogowski_Chunk-List_2017,
 author = {Szelogowski, Daniel},
 doi = {10.48550/arxiv.2101.00172},
 month = {May},
 title = {{Chunk-List}},
 license = {Apache-2.0},
 url = {https://github.com/danielathome19/Chunk-List-rs},
 version = {1.0.0},
 year = {2017}
}
```
