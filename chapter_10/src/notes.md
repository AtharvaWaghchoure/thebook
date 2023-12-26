+ generics for handling concepts duplications

+ Type parameter names in Rust are short, often just a letter and Rust's type-
naming convention is UpperCamelCase.

+ Just use  `T` for *type*

+ Add type parameter name(T,F,ASJDFKDSLFKASJK) before we use it in the function
signature.
    - I meant this `fn func_name<T>(para: T) -> T`
    - Use this similiarly in structs, enums
    - Defining it after the `impl` keyword tells the compiler that the type in
    the angle bracket of the struct is a generic not a concrete type
