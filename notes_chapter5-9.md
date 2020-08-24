# 5. Using Structs to Structure Related Data

If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.


## 5.1 Defining and Instantiating Structs

Structure is defined with keyword `struct` its name and names and types of the pieces of data inside the curley bracket:
```bash
struct User {
    username: String,
    email: String,
    active: bool
}
```

To use struct after we've defined it, we create an instance of that struct by specifying concrete values for each of the fields.
We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs:
```bash
let user1 = User {
    email: String::from("bexxmodd@example.com"),
    username: String::from("bmodd123"),
    active: true
};
```

We can get specific value from a struct by dot notation: `user1.email`. If value is mutable we can change a value:
```
user1.email = String::from("anotheremail@example.com");
```

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
We can aslo construct a new isntance of the struct as the last expression in the function body to implicitly return that new isntance.
```bash
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
}
```

Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user:
```bash
fn build_user(email: String, username: String) -> User {
    User {
    email,
    username,
    active = true,
    }
}
```
This way we created a new instance of the `User` with passed `email` and `username` as parameters.


t’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some. You’ll do this using struct update syntax:
```bash
let user2 = User {
    email: String::from("user2@example.com"),
    username: String::from("user2isHere"),
    active: user1.active,
};
```

We can also achieve the same with less code (even if we are copying several argument values).
The syntax `..user1` specifies that the remaining fields should have the same value as the fields in the given instance.

You can also define structs that look similar to tuples, called _tuple structs_. Those structs have no name associated with their fields, rather just the types.
Tuple structs are useful when you want to give a tuple name and it be different from other tuples:
```bash
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
```
Each struct you define is its own type, even though the fields within the struct have the same types.


We used the owned String type rather than the &str string slice type.
This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid

It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes.


## 5.2 Example Program

Let's write a program that calcualtes the area of rectangle.

We use struct to do that:
```bash
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }

    println!("{} has an area of {}", rect1, area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance
we want to borrow the struct rather than take ownership of it. This way `main` retains its ownership and can continue using `rect1`.
That's why we use `&` in the function signature and where we call the function.

### Adding functionality
When printing complex data types, inside of curly brackets of `println!()` you can add `{:?}` for full display of it of `{:#?}` for pretty print.
But Rust doesn't include functionality to print out debugging infromation, we have to explicitly opt in to make available.
To do that we add the annotation `#[derive(Debug)]` just before the struct definition:
```
#[derive(Degub)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

## 5.3 Method Syntax

Methods are defined inside of structure or enum and their first parameter is always `self`, which represents the isntance of the struct the method is being called on.

```bash
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

To define the function within the context of `struct` we start an `impl` (implementation) block. Then the function block is inside that `impl`.
We’ve chosen `&self` here for the same reason we used `&Rectangle` in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it.

If want to change the instance that we've called the method on ad part of what the method does, we'd use `&mut self`.
Using `self` directly instead of its reference is rare and is done when you want to transform `self` into something and prevent the caller from using the original isntance after that transformation.
you can pass just `self` and consume the struct and free up memory.


### Associated Functions
We’re allowed to define functions within `impl` blocks that don’t take `self` as a parameter.
These are called _associated functions_. They're not methods but functions as they don't have an instance of the struct to work with.

Associated functions are often used for constructors that will return a new instance of the struct:
```bash
impl rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```
To call associated functions we use `::` syntax. `let sq = Rectangle::square(3);`

We can also have multiple `impl` blocks. 


# 6. Enums and Pattern Matching

Enums allow you to define a type by enumerating its possitble variants.

## 6.1 Definint an Enum
let's for example define `enum` for ip address, one with 4 digit and one with 6 digits:
```bash
enum IpAddrKind {
    V4,
    V6,
}
```

We can create instances of each of the two cariants like this:
```bash
let four = IpAddrKind::V4;
let six = IpAddrKind::v6;
```

We can then define a function that takes any `IpAddrKind` value: `fn route(ip_kind: IpAddrKind) {}`
This can be combined with `struct` to identify and store the data:
```bash
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.01");
}
```

We can represent the same concept in more coincise way by just using enum by puting data directly into each enum variant.
```bash
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
```
Here we say that enum `IpAddr` will have associated `String` values. Each type can also have different types and amounts of associated data.
For example: `V4(u8, u8, u8, u8)` and `V6(String)`.

Another example:
```bash
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
This enum has four variants with different types:
* `Quit` has no data associated with it at all
* `Move` includes an anonymous struct inside it.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32` values.

There is one more similarity between enums and structs: just as we’re able to define methods on structs using `impl`, we’re also able to define methods on enums.
```bash
impl Message {
    fn call(&self) {
        // Method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

Rust doesn't have **Null** value but it has an enum that can encode the concepts of a value being present or absent.
This enum is `Option<T>` and is defined by the standard library as follows:
```bash
enum Option<T> {
    Some(T),
    None,
}
```

You can use `Some` and `None` directly. The syntax `<T>` is a generic type parameter. It means the `Some` variant of the `Option` enum can hold one piece of data of any type.
Examples of using `Option` values:
```bash
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
If we use `None` rather than `Some`, we need to tell Rust what type of `Option<T>` we have.

In short, because `Option<T>` and `T` (where `T` can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value.
This example will end up throwing an error:
```bash
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```









