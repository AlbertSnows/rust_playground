An **expression** produces a value. A **pattern** matches and destructures a value.

Expressions — things that evaluate to something:
```rust
let r = &x;      // &x is an expression — creates a reference
let v = *r;      // *r is an expression — dereferences
let y = x + 1;   // x + 1 is an expression
```

Patterns — things that appear on the left side of `=` or in function parameters:
```rust
let &v = r;           // &v is a pattern — matches a reference
let (a, b) = pair;    // (a, b) is a pattern — destructures a tuple
fn foo(&(a, &b): ...) // &(a, &b) is a pattern — in a parameter
```

The easy rule: if it's on the **left** side of `=` or in a `match` arm or function parameter, it's a pattern. If it's on the **right** side, it's an expression.

```rust
let PATTERN = expression;
```

example:
&(param1, &param2): &(type1, &type2) = param1, param2
this is called destructuring
