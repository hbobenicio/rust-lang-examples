# concurrency

## Closure Captures

A closure can capture its environment (stack context) in 2 ways:
- By Reference (default) - Borrows the environment
- By Move - Takes ownership of the environment

## 1:1 vs M:N Threading

1:1 - One Programming language thread for One OS Thread (Rust uses it)
M:N - Also called Green Threads. More Runtime footprint. (External crates can impl this)

## Which thread has all the data?

this:

```rust
fn foo() {
  let data = ...;

	thread::spawn(|| {
	   // Thread T never uses `data`!
	}
	// Main Thread owns `data`
}
```

or this:

```rust
fn foo() {
  let data = ...;

	thread::spawn(move || {
	   // Thread T owns `data`!
	}
	// Main Thread never uses `data`
}
```

You need to tell which thread has the data/environment. Only one of them can have it.

