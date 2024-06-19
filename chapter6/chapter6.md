enums give you a way of saying a value is one of a possible set of values.

If we use enums, the following code becomes

```rust
enum IpAddKind {
    V4,
    V6
}

struct IpAdd {
    kind: IpAddKind,
    address: String
}

let home = IpAdd {
    kind: IpAddKind::V4,
    address: String::from("10.0120.1")
}

let loopback  = IpAdd {
    kind: IpAddKind::V6,
    address: String::from("::1")
}
```

more concise like this:

```rust
enum IpAdd {
    V4(String),
    V6(String)
}

let home = IpAdd::V4(String::from("0.201.123"));

let loopback = IpAdd::V6(String::from("::1"));
```

## Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match conin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
```

This seems very similar to a conditional expression used with `if`, but there’s a big difference: with `if`, the condition needs to evaluate to a Boolean value, but here it can be any type. The type of `coin` in this example is the `Coin` enum that we defined on the first line.

The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Only when we have an `Option<i8>` (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

Matching with Option<T>

```rust
 fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

We didn’t handle the `None` case, so this code will cause a bug.