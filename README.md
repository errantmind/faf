# faf
FaF (Fast as Fuck) is a Linux webserver written in Rust. It has a single goal: to demonstrate the upper bound of possible single-node performance. It is meant as a living reference project and may have cutting edge dependencies. Being a reference project, documentation and simplicity are essential and will be maintained to the best of my ability.

The FaF source code currently has no dependencies and can be converted into a #![no_std] project. See `Code Tour` below for more info.

[FaF Repo](https://github.com/errantmind/faf)

## Requirements and How-To

FaF requires:
* linux x86_64
* nightly Rust
* [clang-13 and lld-13](https://apt.llvm.org/) to be installed and available in PATH. The version (i.e. 13) may change over time as Rust's `rustc` updates its LLVM version


To use FaF for your own purposes, provide a callback which modifies the response buffer. From the callback, return the number of bytes written to the buffer. The callback will be called once per HTTP request. See the [FaF Example Project](https://github.com/errantmind/faf-example) for more information

## Design Principles
1. Speed. Optimize for serving small to moderate payloads to a large number of concurrent connections. Speed will be balanced against over-specialization, like rewriting the entire project in hand-optimized assembly. Speed optimizations are constrained to unsafe Rust, unless the results of some alternative approach are overwhelmingly convincing.
2. Elegance as Simplicity. Consistency in the architecture, project structure, idioms, and style. Some of these idioms and styling are particular to my taste. Will use a 'right tool for the right job' approach when choosing data structures and algorithms.
3. Memory Safety. This being third is generally at odds with the Rust community, but if you have read all the text above you will understand. Memory safety is prioritized in-so-far as it has no decernable effect on performance. It should also be implemented in such a way that doesn't add excessive complexity / indirection in the project.

(in that order)

## Code Tour

Just look at `epoll.rs`, everything is either there or referenced there and, even then, it is only ~230 lines of code.

Aside: a `no_std` version of this project compiles to a total of only ~400 lines of assembly TEXT, and 7KB binary, although it takes a few modifications to get there: the only real dependency on std is threading, so if we eliminate it and change to a `1 process per core` model instead of `1 thread per core` we get a very minimal setup. The performance is ~1% worse. If you are interested in this, let's discuss.

## Decisions
* Use epoll for event loop as io_uring is not fully stabilized (as of 2021/03/24) and io_uring may not actually be faster than epoll in all cases. At some point in the future I will do a test and rewrite FaF using io_uring, but not until sometime in 2022.
* Strive to minimize system calls as they, in aggregate, can cause significant slowdowns
* Use separate epoll instances on separate threads, one per CPU core, to handle all the 'work' (read / writes)
   * Messing with priority and affinity of these threads negatively impacted performance (20+% reduction)
   * Numa is not an issue in the current test environment so less gains to be had with thread affinity
   * I originally had a single thread that dispatched the connections to different worker threads, but it didn't efficiently handle large numbers of incoming connections (16k). I changed this to where each worker thread both accepts connections and handles the work for those connections, one worker thread per core
* Minimize dependencies, even if that means rewriting or stripping other projects
   * Some potential dependencies are less optimized than they could be, which turns out to be most of them. A simple example of this is not marking functions as `#[inline]`, which, in Rust, ends up preventing them from being inlined across crate boundaries
   * Some potential dependencies have 'nasty hacks' which break performance optimizations (like LTO)
* Nothing should be converted to a string that doesn't need to be. Put another way, everything that can remain in byte (u8) sequence should stay that way. For example, why do most frameworks convert the date to a string and then back to a byte sequence? This is unnecessary work
* Use SIMD (SSE4.2) for parsing the request type and path out of the HTTP request

## Contributions
Contributions are welcome, but please discuss before submitting a pull request. If a discussion leads to a pull request, please reference the \#issue in the pull request. Unsolicited pull requests will not be reviewed nor merged.

Any and all contributions, unless otherwise specified, will be licensed under the license specified by this project (below). All contributors agree to automatically and irrevocably relicense their code to match the project license if the project license changes. All contribitors, by submitting contributions as a pull request (or in any other form), agree to these terms. Any license changes will occur as a bump in versioning and the previous license will remain in effect for the previous version.


## License
All code is licensed under AGPL 3.0 unless an individual source file specifies otherwise.

If you don't like the license, convince me.
