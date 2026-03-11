# The Rustonomicon

Source: https://doc.rust-lang.org/nomicon/print.html
Retrieved: 2026-03-10

---

The Rustonomicon is The Rust Programming Language's official guide to unsafe Rust. It documents the details necessary for writing unsafe Rust programs correctly.

> **Warning:** This book is incomplete. See the issue tracker for what's missing or outdated.

## Purpose

Unlike The Rust Programming Language, this book assumes considerable prior knowledge of systems programming and Rust. It's a high-level companion to The Reference, describing how to use language pieces together and the issues that arise.

**Topics covered:**
- Meaning of (un)safety
- Unsafe primitives (language and stdlib)
- Techniques for safe abstractions with unsafe primitives
- Subtyping and variance
- Exception-safety (panic/unwind-safety)
- Working with uninitialized memory
- Type punning
- Concurrency
- FFI (Foreign Function Interface)
- Optimization tricks
- How constructs lower to compiler/OS/hardware primitives
- How to avoid making the memory model angry

---

## Meet Safe and Unsafe Rust

Rust contains both **Safe Rust** and **Unsafe Rust**:

- **Safe Rust**: True Rust programming language. Guarantees type-safety and memory-safety. No undefined behavior possible.
- **Unsafe Rust**: Lets you do "really unsafe" things, while still being more controlled than C.

The **soundness property** of Safe Rust:

> **No matter what, Safe Rust can't cause Undefined Behavior.**

---

## How Safe and Unsafe Interact

The `unsafe` keyword acts as an interface between Safe and Unsafe Rust.

### Uses of `unsafe`

1. **On functions and trait declarations**: Declares the existence of contracts the compiler can't check
2. **On blocks**: Declares that all unsafe actions within maintain the contracts of those operations
3. **On trait implementations**: Declares the implementation upholds the trait's contract

### What Unsafe Rust Can Do

Only five things differ in Unsafe Rust:

1. **Dereference raw pointers**
2. **Call `unsafe` functions** (C functions, compiler intrinsics, raw allocator)
3. **Implement `unsafe` traits**
4. **Access or modify mutable statics**
5. **Access fields of `union`s**

### Undefined Behavior Causes

1. Dereferencing dangling or unaligned pointers
2. Breaking pointer aliasing rules
3. Calling functions with wrong ABI or unwinding with wrong unwind ABI
4. Data races
5. Executing code with unsupported target features
6. Producing invalid values:
   - `bool` not 0 or 1
   - `enum` with invalid discriminant
   - Null `fn` pointer
   - `char` outside valid Unicode ranges
   - Integer/float/raw pointer from uninitialized memory
   - Reference/`Box` that is dangling, unaligned, or invalid

### Rust's Unsafe Traits (as of 1.29.2)

- `Send`: Safe to move to another thread
- `Sync`: Threads can safely share via shared reference
- `GlobalAlloc`: Customize program's memory allocator

---

## Working with Unsafe

### The Non-Local Nature of Safety

**Example of non-local unsoundness:**

```rust
fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx <= arr.len() {  // WRONG: should be <
        unsafe {
            Some(*arr.get_unchecked(idx))
        }
    } else {
        None
    }
}
```

Changing only "safe" code (`<` to `<=`) breaks the soundness of the unsafe code. **Safety is non-local.**

### Module Boundaries and Privacy

Privacy at module boundaries is the solution. Only code in the same module can access private fields or call private functions. This limits the scope of what unsafe code must trust.

```rust
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}
```

Private fields ensure only `Vec`'s own code can manipulate the invariants that the unsafe code depends on.

---

## Data Representation in Rust

### repr(Rust) - Default

- Order, size, alignment are unspecified for optimization
- Alignment: must be at least 1 and always a power of 2
- Type's size must always be a multiple of its alignment

### repr(C)

- Most important repr for FFI
- Order, size, alignment exactly as C/C++
- Use for types crossing FFI boundaries
- Tools: rust-bindgen, cbindgen

### repr(transparent)

- Layout and ABI identical to the one non-zero-sized field
- Use case: transmuting between field type and wrapper type
- Example: `UnsafeCell<T>` can transmute to `T`

### repr(u\*) / repr(i\*)

- For fieldless enums: specifies discriminant size and sign
- Suppresses null pointer optimization when used with enums that have fields

### repr(packed) / repr(packed(n))

- Forces alignment of at most `n` (or 1)
- Strips padding
- **Warning:** Can cause undefined behavior; use sparingly

### repr(align(n))

- Forces alignment of at least `n`
- Use case: prevent false sharing in concurrent code

### Null Pointer Optimization

`Option<&T>` (single unit variant + non-nullable pointer) = size of `&T`

Works for: `Box<T>`, `Vec<T>`, `String`, `&T`, `&mut T`

### Exotically Sized Types

#### Dynamically Sized Types (DSTs)

Types without statically known size. Only behind pointers (wide pointers):
- Trait objects (`dyn MyTrait`): vtable pointer
- Slices (`[T]`, `str`): element count

#### Zero Sized Types (ZSTs)

Types occupying no space. Operations on ZSTs reduce to no-ops.
- `Set<Key> = Map<Key, ()>` without overhead
- References must still be non-null and aligned

#### Empty Types

Types that cannot be instantiated (e.g., `enum Void {}`).
- `Result<T, Void>` is infallible
- Compiler optimizes: represented as just `T`

---

## References

Two kinds:

1. **Shared reference:** `&` (many readers)
2. **Mutable reference:** `&mut` (exclusive access)

### Rules

1. A reference cannot outlive its referent
2. A mutable reference cannot be aliased

### Aliasing Matters for Optimization

```rust
fn compute(input: &u32, output: &mut u32) {
    if *input > 10 { *output = 1; }
    if *input > 5  { *output *= 2; }
}
```

Can be optimized (cache `*input` in register) precisely because `&mut` cannot alias `&`.

---

## Lifetimes

Named regions of code where a reference must be valid.

### Elision Rules

1. Each elided input lifetime becomes a distinct lifetime parameter
2. If exactly one input lifetime, assign to all elided output lifetimes
3. If multiple inputs and one is `&self`/`&mut self`, assign self's lifetime to all elided outputs
4. Otherwise, eliding output lifetimes is an error

### Unbounded Lifetimes

Unsafe code can produce references from thin air. Unbounded lifetimes become as large as context demands.

**Example of danger:**
```rust
fn get_str<'a>(s: *const String) -> &'a str {
    unsafe { &*s }  // Unbounded — can produce dangling reference!
}
```

**Best practice:** Use lifetime elision at function boundaries.

---

## Subtyping and Variance

`'long <: 'short` if `'long` completely contains `'short`.

### Variance Table

| Type | Variance over T |
|---|---|
| `&'a T` | covariant |
| `&'a mut T` | **invariant** |
| `Box<T>` | covariant |
| `Vec<T>` | covariant |
| `UnsafeCell<T>` | invariant |
| `Cell<T>` | invariant |
| `fn(T) -> U` | **contravariant** over T |
| `*const T` | covariant |
| `*mut T` | invariant |

**Why `&mut T` is invariant:** Allows exclusive modification; subtyping would allow assigning shorter-lived references into longer-lived slots — use-after-free.

---

## Drop Check

**The Big Rule:** For a generic type to soundly implement `Drop`, its generic arguments must **strictly outlive** it.

Without this, destructors could access already-freed memory.

---

## Uninitialized Memory

- `MaybeUninit<T>` is the safe abstraction for uninitialized memory
- Reading uninitialized memory is undefined behavior
- Use `assume_init` only when certain memory is initialized

---

## Type Conversions

### `transmute`

```rust
fn transmute<T, U>(t: T) -> U
```

Reinterprets bits of `T` as `U`. **Incredibly unsafe** — only use with strong justification.

---

## Concurrency

- **`Send` trait**: Type safe to move to another thread
- **`Sync` trait**: Type safe to share between threads
- **Atomic types**: `AtomicBool`, `AtomicUsize`, etc.
- **Mutexes and RwLocks**: Safe synchronized access

---

## FFI (Foreign Function Interface)

- All FFI functions are `unsafe`
- Types crossing boundaries should use `repr(C)`
- Use tools: rust-bindgen, cbindgen
- Carefully verify contracts with C code
- Never use enum types directly across FFI boundaries without `repr(C)`

---

## Key Takeaways

1. **Safety is non-local**: Soundness depends on module-wide invariants
2. **Privacy is protective**: Limits scope of unsafe code that must be trusted
3. **Lifetimes and ownership ensure memory safety** in Safe Rust
4. **Unsafe Rust requires discipline**: Understand the contracts you're upholding
5. **Use unsafe sparingly**: Most code should be Safe Rust
