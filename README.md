[Tutorial](https://doc.rust-lang.org/book/ch00-00-introduction.html) [GitHub](https://github.com/capsci/rust-tut)

**Foreword**
TBH, I wanted to start the day with [Zig](https://ziglang.org/); but after seeing a lot of promises about rust community and maturity; I decided to pursue Rust first.
Besides with Linux kernel adopting Rust(i know it was controversial); I was more comfortable with placing by bet on Rust. Also, many developers I follow online transitioned to Zig via Rust and I don't mind following the same route and learn things along the way.
That being said, lets begin.

> [!Info] Installing `rustup` and other binaries
```
brew install rustup
# did not install rustup in ~/.cargo/bin; but in homebrew's bin directory
rustup default stable
# did not install binaries in ~/.cargo/bin; but in ~/.rustup/toolchains/stable-{arch}/bin
```
> [!tip] add following to `~/.zshrc`/`~/.zshrc.mine` to bring appropriate binaries on the path
```zsh
export PATH=$PATH:~/.rustup/toolchains/stable-aarch64-apple-darwin/bin
```

We can determine places where `rustup` downloads by setting environment variable `RUSTUP_HOME`

> [!Error] Could not get tab completions working :(
```
rustup completions bash > $(brew --prefix)/etc/bash_completion.d/rustup.bash-completion
echo "fpath+=~/.zfunc" >> ~/.zshrc
```

> [!Todo] Skipping learning `rustup` for now; focusing on **Rust** 

# Getting Started

Low-level programming is prone to subtle bugs which is predominantly handled by extensive testing and careful code review; with Rust, the compiler plays a gatekeeper role refusing any code with elusive and concurrency bugs.

> [!Info] `rustfmt` is used by Rust projects to format code to a specific style in your project

## Hello World
```rust
println!("Hello, world!");
```
In above `println!` is a rust macro; functions in Rust are called without `!`

## Hello Cargo

> [!Info] `cargo` is Rust's build-system and package manager

```
# create a new Rust project with specified Version Control System
cargo new hello_cargo --vcs=git

# build using cargo; creates executable target/debug/hello-cargo
cargo build

# or alternatively use below command to build and run the executable
cargo run

# use below to check if program is compilable
# we wont generate an executable but its faster than `cargo run/build`
cargo check

# creates a release with "optimizations"
cargo build --release
```

> [!Info] In Rust, packages of code are referred as `crates`

# Programming a Guessing Game

Rust has a set of items (called *prelude*) which are defined in standard library and are brought in scope for every program.
For things not in *prelude*; you'll have to bring them to scope explicitly using `use` statement.

In Rust, variables are immutable by default.

`cargo update` updates the crate while ignoring the `Cargo.lock` file; to figure out the latest version that satisfies `Cargo.toml`

> [!Info] Rust allows **shadowing** of variable names

# Common Programming Concepts
## Variables and Mutability

- variables are immutable by default; add `mut` keyword to make them mutable.
- constants are **always** immutable; and cannot be set to value thats computed at runtime.
 
- **Shadowing** variables allow up to update their values :(
	- we need `let` keyword to shadow a variable
> [!Error] Why did Rust allow Shadowing? doesn't it defeat the usefulness of immutability

## Data Types
### Scalar
#### integers
- signed types start with `i`; unsigned types start with `u`
- number literals can use `_` as visual separator (eg. `1_000` is read as `1000`)
- numbers can be decimal, hex, octal, binary or byte(`u8` only)
- rust defaults integer to `i32`
- Integer Overflow
	- debug mode will "panic"
	- release mode will wrap :(
	- use standard libraries methods on primitives to explicitly handle this
		- `wrapping_*`, `checked_*`, `overflowing_*` & `saturated_*` methods
> [!Error] **IntegerOverflows** have different behavior in "debug" vs "release" mode
#### floating-point numbers
* `f32` and `f64` types
* `f64` is default
#### booleans
* allows `true` and `false`
#### characters
* 4 bytes in size and represents unicode scalar value
### Compound
#### tuple
* Grouping number of values of different types into 1 compound type
#### array
* stores multiple values of same type
* arrays in Rust have fixed length
	* **vector** is similar collection provided by standard library which is allowed to grow/shrink in size
## Functions
* Rust uses snake_case as conventional style for function and variable names
* `statements` are instructions that perform some actions and do not return a value
* `expressions` evaluate to a resultant value
	* expressions do end with a semicolon `;`
## Control Flow
### loop
> [!Error] why not use `while` more efficiently instead of using `loop` ?
- "designed" to retry operation which might fail
- loop labels must begin with single quotes
### while
### for
# Ownership
Ownership enables Rust to make memory safety guarantees without needing garbage collector.
In Rust, memory is managed through system of ownership with a set of rules that compiler checks.

Data can be stored on a stack or heap.
All data stored in stack must have known fixed size.
Data with unknown size at compile time(or size that might change) should be stored in heap.
Pushing on stack is faster than allocating on heap; since allocator does not need to find next available (big enough)space on the heap.
Keeping track of parts of data which uses heap; minimizing duplicates and removing unused data are the problems that ownership addresses.

**Ownership Rules**
* Each value has an owner.
* There can be only one owner at a time.
* When owner goes out of scope, the value will be dropped.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is no longer accessible from this point on
// when s1 goes out of scope; the memory is no longer freed (ps - s2 is using the same location)
```
Rust never creates *deep* copies of data; it simply moves.
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // s1 remains accessible
```

```rust
let x = 5;
let y = x; // both x and y are accessible, since integers are known size during compilation and above assignments uses stack
```

Rust also allows transferring value without ownership, controlled using *references*.

## References and Borrowing

`&s` syntax creates a reference which refers to value of `s` but does not own it.
Hence the value pointed by it does not drop when reference is stopped being used.

References are immutable and we are not allowed to update its value
## Slices

> [!Info] String literals are stored as immutable slices
