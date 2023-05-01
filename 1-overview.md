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
   - define types before assignment (errors at compile time)
   - types are determined at assignment (error at runtime)

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

Primitives
 - strings
 - floats
 - integers
 - booleans


### Functions

