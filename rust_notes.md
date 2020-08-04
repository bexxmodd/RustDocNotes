## 1. Cargo

If you started a project that doesn’t use Cargo, you can convert it to a project that does use Cargo.
Move the project code into the `src` directory and create an appropriate `Cargo.toml` file.

* `cargo build` : to build a project. use `--release` to build final program ready for production.

* `cargo check` : to check your code to make sure it compiles but doesn't produce an exe.

* `cargo run` : to compile the code and then run the resulting executable all in one cmd.


## 2. A Guessing Game

* `use std::io;` : to import input/output library, for example, from the standard library (std).

* `fn <name()>` : declares a new function.

* `println!` : is macro that prints a string on the screen.

* `let` statement is used to create a variable. By default variables in Rust are immutable. To make variable mutable add `mut`.

For example let's create an mutable variable which will be a string.
```
let mut bar = String::new();
```

`::` syntax indicates that new is an associated function of the string type.

* `&` indicates that the argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
my default _referances_ are mutable so to make _reference_ mutable you need to use `&mut <varname>`.

Many types are named `Result` in std. `Result` types are _enumarates_, often refered as *enums*.
An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
For `io::stdin` `Result`, the variants are `ok` or `Err`. `ok` indicates that operation was success and `Err` that operation failed.

Remember that a crate is a collection of Rust source code files. To use outside library we need to add it to `Crate.toml` under the `[dependencies]`.
In case `rand = "0.5.5"` we’ll specify the rand crate with the semantic version specifier 0.5.5.
Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers.
The number 0.5.5 is actually shorthand for ^0.5.5, which means “any version that has a public API compatible with version 0.5.5."

when you first create a project cargo creates a file *Cargo.lock*. When the same project is built again cargo will see that *Cargo.lock* file exists
and will use the dependencies versions indicated stated when the cargo was first created.

* `doc --open command`, will build documentation provided by all of your dependencies locally and open it in your browser.



