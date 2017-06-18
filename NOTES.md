# Rust Learning Notes

### Expressions vs Statements

* Statements have a semicolon ":" at the end;
* Expressions don't have it.

### Functions

* Every function return a expression (normally on the last line);
* "Diverging functions" are the ones that don't return any value.
  * They are declared like so:
  ```rust
  fn foo() -> ! { }
  ```

* You can use Function Pointers like so:
```rust
let f: fn(i32) -> i32;
```

### Primitive Types

* bool (true | false)
* char (literals declared inside single quotes: 'c')
  * Chars in Rust are 4 bytes long, not just 1 (like many other languages)
* (u|i) x (8|16|32|64) ++ (usize, isize, f32, f64)
* arrays
  * Fixed-size list of elements of the same type
  * Has the type [T; N], where T is the element type and N is the array size
  * N is a compile-time constant
  * Syntax sugar for initialization:
  ```rust
  let l = [0; 20]; // l: [i32; 20]
  ```
  * Get it's size with: `l.len()`
  * Get it's element with subscription notation: `l[5] // 0 indexed`
* slices
  * A ‘slice’ is a reference to (or “view” into) another data structure
  * They are useful for allowing safe, efficient access to a portion of an array without copying
  * Slices have a defined length and type &[T]
  * Examples:
  ```rust
  let a = [0, 1, 2, 3, 4];
  let complete = &a[..]; // A slice containing all of the elements in `a`.
  let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
  ```
* str
* tuples
  * A tuple is an ordered list of fixed size
  * Access with pattern match or indexes like so:
  ```rust
  let tuple = (1, 2, 3);
  let x = tuple.0;
  let y = tuple.1;
  let z = tuple.2;
  ```
* function pointers

### Loops

* Infinite loop: `loop { ... }`
* More examples: `for x in expression`, `for x in 0..10`, `for (x = 0; x < 10; x++)`
* `for (index, value) in (5..10).enumerate()`, `let lines = "hello\nworld".lines(); for (linenumber, line) in lines.enumerate()`

### Vectors

* vec![1, 2, 3];

### Strings

* &str (string slice)
  * fixed size
  * cannot be mutated
  * ref to a sequence of UTF-8 bytes
* String (heap allocated string)
  * Growable and UTF-8 also
  * "foo".to_string()
  * For functions that receive a &str trait parameter, use `&*`
  ```rust
  let s = "blabla".to_string();
  foo(&*s);
  ```
* String -> &str: cheap
* &str -> String: not so cheap (requires heap allocation)
* Indexing `s[i]` don't work, because it's a UTF-8 encoded string.
  * `for b in s.as_bytes() { print!("{}, ", b); }`
  * `for c in s.chars() { print!("{}, ", c); }`
  * `let c = "dog".chars().nth(1); // c == 'o'. This traverses the string: O(n)`
* Slicing are BYTE wise:
  * these are byte offsets, not character offsets!
  ```rust
  let dog = "hachiko";
  let hachi = &dog[0..5]; // ok
  let err = &dog[0..2]; // panics!
  ```
* Concatenation:
  * `"Hello ".to_string() + "world"`
  * `"Hello ".to_string() + &("world".to_string())`

### Error Handling

* fn panic!() - Causes the current task to unwind, and in most cases, the entire program aborts
* fn unwrap() - Gives the result of the computation, and if there was an error, panic and stop the program
* enum Option<T> { None, Some(T) } - Express the possibility of absence
  * fn map() - Calls a function on the value of the Option, if present. If it has None, return None simply
  * fn unwrap_or(default_value) - Assigns a default value if None (there is also a fn unwrap_or_else())
  * fn and_then() - Works like flatmap
* enum Result<T, E>
  
