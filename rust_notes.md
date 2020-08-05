# 1. Cargo

If you started a project that doesn’t use Cargo, you can convert it to a project that does use Cargo.
Move the project code into the `src` directory and create an appropriate `Cargo.toml` file.

* `cargo build` : to build a project. use `--release` to build final program ready for production.

* `cargo check` : to check your code to make sure it compiles but doesn't produce an exe.

* `cargo run` : to compile the code and then run the resulting executable all in one cmd.


# 2. A Guessing Game

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

* `match` : expression is made of _arms_. An arm consists of a pattern and the code that should be run if the value given to the beginning of the `match` espression fits that arm's pattern.

When we get the input from the user we can convert it to any type we want. For example if we want to conver to unsigned 32-bit number:
```
let guess: u32 = guess.trim().prase().expect("Please type a number!");
```

Here we rewrite the already existing `guess` value with the new one. Or as we say _shadow_ the previous value.
`guess` refers to the original valua of that variable (that was a string). 

* `trim` : is a method on a string isntance which will eliminate any white spaces at the beginning and end.

* `parse` : method on strings parses a string into some kind of number. Because it can parse into variety of numbers we need to tell exactly what type.

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) = num,
    Err(_) = continue,
}
```
Here we are using `match` again. If `parse` methods is able to convert string to number it `Result`s in `Ok` else it will be `Err`.
Also, `_` is a catchall value; which in this case means that for any type of `Err` we will just continue to the next iteration of the `loop`.


# 3. Common Programming Concepts
## 3.1 Variables and Mutability

Constants are aren't just imuttable by default (meaning you can't use `mut` with them) - they are always immutable.

`const` : to declare a constant, after `let` keyword, and the type of the value must be annotated.

Constants may be set only to a constant expression, not the result of a function call or any other value that can be computed at runtime.

Naming convetion for constatns is to use all uppercase with underscores. Here's example:
```
const MAX_POINTS: u32 = 100_000;
```

-----------
Rustaceans say that the first variable is _shadowed_ by the second, which means that the second variable's value is what appears when the variable is used.
We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword as follows:
```
fn main() {
    let x = 5;
    let x = x + 1;
    println!("x = {x}", x);
}
```
This program first binds `x` to a value of 5 and then it shadows `x` by repeating `let x =`, taking original value and adding 1.

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.


