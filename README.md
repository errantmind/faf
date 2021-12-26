# FaF
FaF (Fast as Fuck) is a Linux webserver written in Rust. FaF..
* has one goal: to demonstrate the upper bound of single-node performance while remaining usable in a production setting.
* is, perhaps, deceptively simple. It is designed to be easily understood while providing cutting edge performance. Each facet of FaF has been meticulously benchmarked and all design decisions were deliberate.
* is open to community contributions that further improve its speed.
* is open to issues that clearly demonstrate a security vulnerability.
* has no Rust dependencies and can be converted into a `#![no_std]` project. See `Code Tour` below for more info. A side-benefit of this is a very small attack surface.

## Requirements and How-To

FaF requires:
* linux x86_64
* nightly Rust
* [clang-13 and lld-13](https://apt.llvm.org/) to be installed and available in PATH. The version (i.e. 13) may change over time as Rust's `rustc` updates its LLVM version


To use FaF for your own purposes, provide a callback which modifies the response buffer. The response buffer will always start empty and have a length as defined in the FaF project, so it is easy to ensure you do not write past this length. If you need more buffer, increase the value of the constant in the FaF web server project. From the callback, return the number of bytes you wrote to the buffer. The callback will be called once per HTTP request. See the [FaF Example Project](https://github.com/errantmind/faf-example) for more information

## Design Principles
1. Speed. Optimize for serving small to moderate payloads to a large number of concurrent connections. Speed will be balanced against over-specialization, like rewriting the entire project in hand-optimized assembly. Speed optimizations are constrained to unsafe Rust, unless the results of some alternative approach are overwhelmingly convincing.
2. Elegance as Simplicity. Consistency in the architecture, project structure, idioms, and style. Some of these idioms and styling are particular to my taste. Will use a 'right tool for the right job' approach when choosing data structures and algorithms.
3. Memory Safety. This being third is generally at odds with the Rust community, but if you have read all the text above you will understand. Memory safety is prioritized in-so-far as it has no decernable effect on performance. It should also be implemented in such a way that doesn't add excessive complexity / indirection in the project.

(in that order)

## Code Tour

Just look at `epoll.rs`, everything is either there or referenced there and, even then, it is only ~200 lines of code.

Aside: a `no_std` version of this project compiles to a total of only ~400 lines of assembly TEXT, and 7KB binary, although it takes a few modifications to get there: the only real dependency on std is threading, so if we eliminate it and change to a `1 process per core` model instead of `1 thread per core` we get a very minimal setup. The performance is ~1% worse. If you are interested in this, let's discuss.

## Contributions
Contributions are welcome, but please discuss before submitting a pull request. If a discussion leads to a pull request, please reference the \#issue in the pull request. Unsolicited pull requests will not be reviewed nor merged.

Any and all contributions, unless otherwise specified, will be licensed under the license specified by this project (below). All contributors agree to automatically and irrevocably relicense their code to match the project license if the project license changes. All contribitors, by submitting contributions as a pull request (or in any other form), agree to these terms. Any license changes will occur as a bump in versioning and the previous license will remain in effect for the previous version.


## License
All code is licensed under AGPL 3.0 unless an individual source file specifies otherwise.

If you don't like the license, convince me.
