Think of lifetimes as **scopes** — a lifetime lasts as long as the block `{}` the value was created in.

```rust
fn main() {
    let x = 5;        // x is born here
    let r = &x;       // r borrows x — r's lifetime tied to x's
    println!("{}", r); // ok, x still alive
}                     // x dies here, r dies here too — fine
```

A problem case:
```rust
fn main() {
    let r;
    {
        let x = 5;    // x born in inner scope
        r = &x;       // r points to x
    }                 // x dies here
    println!("{}", r); // COMPILER ERROR — r points to dead x
}
```

Rust catches this at compile time. The lifetime `'a` is just a name the compiler assigns to that scope so it can reason about it.

For `&'a T` in `slice::Iter` — `'a` is the lifetime of the slice itself. The iterator can only hand out references that live as long as the slice does, which makes sense: if the slice is freed, those references would be dangling.

A useful mental model: **a reference can never outlive what it points to**. Lifetimes are how the compiler enforces that rule.
