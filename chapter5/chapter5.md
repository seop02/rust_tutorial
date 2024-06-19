# BASIC SYNTAX

FLOAT: u64

int: i32

defining variables: let x = something;

no mut —> cannot change this variable x

let y = something;

—> can change this variable y

fn main () is 

```python
if __name__ == '__main__': 
#equivalent in python
```

## class and structure equivalence

In the `User` struct definition in Listing 5-1, we used the owned `String` type rather than the `&str` string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else, but to do so requires the use of *lifetimes*, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct  Color(i32, i32, i32);
struct Pos(u64, u64, u64);

fn build_user(email: String, username: String) -> User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1
    }
}

fn main(){
    let mut user1 = User{
        active:true,
        username: String::from("longtou"),
        email: String::from("hi@gmail.com"),
        sign_in_count:1
    };
    user1.email = String::from("bye@gmail.com");

    let user2 = User{
        active: user1.active,
        username:user1.username,
        email: String::from("good@gmail.com"),
        sign_in_count:user1.sign_in_count
    };
}
```

## Printing debugging information

#[derive(Debug)]

{:#?}

{:?}

use the [`dbg!` macro](https://doc.rust-lang.org/std/macro.dbg.html), which takes ownership of an expression (as opposed to `println!`, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

## methods

All functions defined within an `impl` block are called *associated functions* because they’re associated with the type named after the `impl`. We can define associated functions that don’t have `self` as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the `String::from` function that’s defined on the `String` type.

The following python code is

```python
class Rectangle:
	def __init__(self, height, width):
		self.height = height
		self.width = width
	def compute_area(self):
		area = self.height*self.width
		return area
		
if __name__ == "__main__":
	rect1 = Rectangle(10, 12)
	area1 = rect1.compute_area()
	print(f"area: {area1}")
```

is equivalent to rust code:

```rust
struct Rectangle {
	height:u64,
	width:u64
}

impl Rectangle{
	fn area(&self) -> u64{
		self.width * self.height
	}
}

fn main() {
	rect1 = Rectangle{
	height: 10,
	width: 12
	}
	println!("area: {}",rect1.area())
}
```