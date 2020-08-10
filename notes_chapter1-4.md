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

* `::` syntax indicates that new is an associated function of the string type.

* `&` indicates that the argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
my default _referances_ are immutable so to make _reference_ mutable you need to use `&mut <varname>`.

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

```bash
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
```bash
fn main() {
    let x = 5;
    let x = x + 1;
    println!("x = {x}", x);
}
```
This program first binds `x` to a value of 5 and then it shadows `x` by repeating `let x =`, taking original value and adding 1.

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

## 3.2 Data Types

Rust is a _statically typed_ language, which means that it must know the types of all variables at compile time.

### Scalar Types
A _scalar_ type represents a single value. Rust has four primary scalar types: **integers, float-point numbers, Booleans, and characters**.

**Integers** are unsigned(u) or signed(i) followed by number of bits (8, 16, 32, 64, 128). Signed and Unsigned refer to whether it's possible for the number to be negative or positive.
If negative it needs to have sign so it will be _signed_ integer. Otherwise it's _unsigned_.

Default integer type is `i32` which is recommended when you are not sure which type to use.

**FLoating-Point Types** also has two types, which are numbers with decimal points. We can chose the bits and are denoted with `f` --> Example: `f32`.
default type is `f64` which is recommended.

**The Boolean Type** are only bite in size. It's specified using `bool`.

**Character Type** is called with `char` and is the language's most primitive alphabetic type and they use single quotes oposed to String types which uses double quotes.
`char` type is four bytes in size and represents a Unicode Scalar Value.

### Compound Types
Rust has two primitive compound types: tuples and arrays.

**Tuple** is a general way of grouping together a number of values with a variety of types into one compound type.
Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position inside has a type.
Example with optional annotation:
```bash
fn main() {
    let tup: (i32, f64 ,u8) = (500, 6.4, 1);
}
```

Becayse a tuple is a single compound element. To get the individual values out of a tuple, we can use pattern matching to _destructure_ a tuple value:
```bash
fn main() {
    let tup = (500, 6.1, 4);

    let (x, y, z) = tup;
}
```

In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (.) followed by the index of the value we want to access.
For example: `let six = x.2;`.

**Array** is another way to have multiple values in a collection. Unlike a tuple every element in the tuple should have a same type.
They also have fixed length like tuples. Values are written in array as comma-seperated list: `let a = [2, 3, 1, 7]`.

Arrays are useful when you want your data allocated on the stack rather than the heap.
A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

Good example of when to use array rather than a vector is in a program that needs to know the name of the months of the year.
It's unlikely that you will need to add or remove month so it will always contain 12 elements.

Array is a single chunk of memory allocated on the stack and you can access elements in it using indexing:
```bash
let a = [1, 5, 3 ,6];
let first = a[0];
let second = a[1];
```

## 3.3 Functions

Rust uses _snake case_ as the conventional style for function and variable names.
function definition in Rust starts with `fn`.
Rust doesn’t care where you define your functions, only that they’re defined somewhere.

**Function Parameters** can be provided to the Rust functions, which are called _arguments_ sometimes _parameters_.
For example:
```bash
fn another_function(x: i32, y: i32) {
    println!("First parameter is {} and the second {}", x, y);
}
```

In function signature you must declare the type of each parameter.

Function bodies are made up of a series of statements optionally ending in an expression. Rust is an expression-based language

_Statements_ are instructions that perform some action and do not return a value. Function definitions are also statments.
_Expressions_ evaluate to a resulting value. Expressions do not include semicolons. If you add semicolon at the end you'll turn it into a statement.

We declare return value type after an arrow `->`.
The return value of the function is synonymous with the value of the final expression in the block of the body of a function.
You can return early from a function by using `return` keyword and specify a value, but mostly functions return the last expression implicitly.
```bash
fn square(x: i32) -> i32 {
    x * x
}
```

## 3.4 Comments

You can leave comments by starting with two forward slashes -> `// Some comment``. Comments only continue till the end of the line.
Another type of comments, documentation comments, will be discussed in Chapter 14.


## 3.5 Control Flow

The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

n `if` expression allows you to branch your code depending on conditions. `if` expressions are sometimes called _arms_.
Example of if-else syntax:
```bash
if x < 5 {
    println!("Low score!");
} else {
    println!("Good score!
}
```

It’s also worth noting that the condition in this code must be a `bool`.
You can have multiple conditions with `else if` expression.

Because `if` is an expression, we can use it on the right side of a `let` statement:
```bash
let condition = true;
let number = if condition { 5 } else { 6 };

println!("The value of number isL: {}", number);
```

the values that have the potential to be results from each arm of the if must be the same type.

Rust has three kinds of loops: `loop`, `while`, and `for`.

The `loop` keyword tells Rust to execute a block of code over and over again forever or until we stop it explicitly by keyword `break`.
You can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:
```bash
let mut counter = 0;
let result = loop {
    count += 1;
    if counter == 10 {
        break counter * 2; // Will return 10 * 2 or 20
    }
}
```

Next one is the conditional loop `while` which has very familiar syntax: `while i < 0` etc.
You could use the `while` to loop of the elements of a collection, such as an array.
This is also slow, because the compiler adds runtime code to perform the conditional check on every element on every iteration.

Better alternative is `for` loop. For example:
```bash
let a = [10, 20, 30, 40];
for i in a.iter() {
    println!("{}",i);
}
```
As showed we need to at `.iter()` to the array before looping over it.
Another useful method is `.rev()` which reverses the range. Example: `for i in (1..5).rev()`

# 4. Understanding Ownership

Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
Therefore, it’s important to understand how ownership works in Rust.


## 4.1 What is Ownership?

In Rust memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
Ownership features do not slow doen your program.

### Stack and Heap

Both of them are parts of memory that are available to your code to use at runtime, but they are structured in different ways.
The _stack_ stores valye in the order it gets them and removes the values in the opposite order. ALso referred as _last in, first out_.
Adding data is called pushing onto the stack and removing data is called popping off the stack.
All data stored on the stack must have a know, fixed size. Data with an uknown size at a compile time or a size that might change must be stored on the heap instead.

The _heap_ is less organized: when you put data on the heap, you request a certain amount of space.
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and retunrs a pointer, which is the address of that location.
This process is called allocating on the heap or just allocating.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; it's always on top.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.


Ownership rules:
* Each value in Rust has a variable that's called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the valyue will be dropped.

A scope is the range within a program for which an item is valid.

All the data types covered in chapter 3 are stored on the stack and poped of the stack when their scope is over.
String is stored on the heap. You can create a `String` from a string literal using the `from` function:
```bash
let s = String::from("Hello");
```

The double colon (`::`) is an operator that allows us to namespace this particular `from` function under the `String` type.
In Rust, when something is stored on the heap (and memory is allocated to it) the memory is automatically returned once the variable that owns it goes out of scope.
```bash
{
    let s = String::from("Hello"); // s is valid from this point forward
    // do stuff with s
}   // this scope is now over, and s is no longer valid
```

Rust automatically calls `drop` function for us after the closing the curly bracket.

example:
```bash
    let s1 = String::from("hello!");
    let s2 = s1;
```
You will assume that the `String` hello is bind to `s1` and makes a copy of the value and bind to `s2`. But this isn't quite what happens.
This group Take a look at Figure 4-1 to see what is happening to String under the covers. A String is made up of three parts, shown on the left:
a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
![](https://doc.rust-lang.org/book/img/trpl04-01.svg)

The length is how much memory, in bytes, the contents of the `String` is currently using. Capacity is the total amount of memory that the `String` has received from the allocator.

When we asssign `s1` to `s2` `String` data is copied, meaning we copy the pointer, the length, the capacity that are on the stack.
We do not copy the data on the heap that the pointer refers to. It looks like this:

![](https://doc.rust-lang.org/book/img/trpl04-02.svg)

Rust also invalidates the first variable, instead of being called a shallow copy, it's known as _move_. With above example we would say that `s1` was _moved_ into `s2`.
This is what actually happens:
![](https://doc.rust-lang.org/book/img/trpl04-04.svg)

If we want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone()`:
```bash
let s1 = String::from("hello");
let s2 = s1.clone();
```

Also the types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
There are referred as `Copy` types and some examples are: all `integer`, the `boolean` types, all the `float`, the `char`, and `Tuples` only if they contain that are also `Copy` type.

## 4.2 References and Borrowing

This is how you will define a `calculate_length` function that has a reference to an object as a parameter instead o taking ownership of the value:
```bash
fn main() {
    let s1 = String::from("hello");
    let len = calcualte_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len();
}
```
Notice, we pass `&s1` into function and we take `&String` rather than String. and also the return value is gone.
This are references and they allow you to refer to some value without taking ownership of it.
![](https://doc.rust-lang.org/book/img/trpl04-05.svg)

Because we have reference not a copy, where `&s1` refers to `s1`, `s1` will not be droped if the reference goes out of scope.

We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

If we want to return mutated value of a string from a function we need to use `&mut`. Example:
```bash
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.

The benefit of having this restriction is that Rust can prevent data races at compile time. A _data race_ is similato to a race conditions and happens when:
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

If we try to return a reference to the string with the function we'll get an error, because the string will be dropped when it goes out of scope (end of function)
and refernce is pointing to a none existing string. In that case you need to return actual string instead of the reference.


## 4.3 The Slice Type

Another data type that doesn't have ownership is the _slice_. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
Remember `iter` is a method that returns each element in a collection and that `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead.
For example we can iterate over the string and return each char and its index by:
```bash
for (i, &item) in string.iter().enumerate()
```

A _string slice_ is a reference to a part of a `String`, and it looks like this:
```bash
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This is a reference to a portion of the string. We can create ranges within brackets by telling `[starting_index..ending_index]` :
~[](https://doc.rust-lang.org/book/img/trpl04-06.svg)

if you start from the 0 index you can drop zero and use it like this: `let slice = &s[..2];`. Similarly no need to indicate the last index value.

Example function which returns a slice:
```bash
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
If we have an immutable reference to something, we cannot also take a mutable reference. Because `clear` needs to truncate the `String`,
it needs to get a mutable reference.

For example `let s = "Hello, world!";` here `s` is `&str`: it's slice pointing to that specific point of the binary. And they are immutable.

There also are other slices. For example integer slices `let a = [2, 4, 2, 6];` we can slice like this: `let slice = &a[..2];`
