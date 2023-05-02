## Overview

Rust compiles to either machine code or WebAssembly.

You can build with Rust:

 - web servers
 - command-line interfaces
 - native desktop applications
 - in-browser app (WebAssembly)
 - Operating Systems

Rust is all about performance (c/c++ level performance).

Rust has a built-in package manager and code formatter.

Rust can allows for a dynamically typed and statically types:
   - define types before assignment
   - types are determined at assignment (type inference)

Rust drawbacks:
 - large codebase to learn
 - strict compiler (i.e. borrow checker errors)
 - slow compiler (i.e. long test build)
 - Elm is a good alternaive if speed is not top priority

### Variables and Data Types

By default, variables are immutable. When a variable is immutable, once a value is bound to a name, you can’t change that value.

Will cause and error of: cannot assign twice to immutable variable `x`

```
let x = 5;
x = 6;
```

But you can make them mutable by adding `mut` in front of the variable name:

```
let mut x = 5;
x = 6;
```

**Constants**

Constants are always immutable. You aren’t allowed to use `mut` with constants. 

**Shadowing**

Shadowing allows you to re-declare a variable in the same scope, using the same name. The re-declared variable differs from the original by having a different type. This is especially useful upon casting data from one type into another.

```
let a = ""
let a = "banana"
 
println!("{}", a); // ---> "banana"
```

**Primitives**

  Strings: 
  
  A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated. 

  `let name:string = "frank';

  Floats: 
  
  Rust has two different sizes of float: f64 and f32. The former is 64 bits (8 bytes) large, whereas the latter is 32 bits (4 bytes) large. 

  The trade-off is that f64 can store more digits, making it more precise and able to represent larger numbers, but it takes up more memory. This memory difference can really add up in applications that store massive quantities of floating-point numbers. For example, in 3D games, f32 is very commonly used instead of f64.

  ```
    let x: f64 = 1.1;
    let y: f32 = 2.2;
    let z = x * y; // error incompatiable types
  ```
  
  Integers:

  Rust integers also come in different sizes, and those sizes are reflected in their types.  


 
  Booleans


### Functions

```
fn multiply_both (x: f64, y: 64) -> f64 {
    retun x * y;
}
```