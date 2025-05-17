# utilz

**A zero-dependency extension trait crate for Rust.**
Sick of writing `if let Some(x)` all the time? Want `.tap()`, `.toggle()`, or `.if_ok()`?
This crate gives you clean, readable, expressive utility extensions without the bloat.

---

## Features

> Lightweight extensions for common types like `Option`, `Result`, `Vec`, `bool`, `str`, `HashMap`, and more!

| Trait           | Description |
|----------------|-------------|
| `OptionUtils`  | Ergonomic handling of `Option<T>`: `.if_some()`, `.or_default_with()` |
| `ResultUtils`  | Convenient sugar: `.if_ok()`, `.if_err()`, `.unwrap_or_exit()` |
| `BoolUtils`    | Fancy conditionals: `.not()`, `.toggle()`, `.if_true()` |
| `VecUtils`     | Conditional pushes: `.push_if()`, `.push_if_with()` |
| `MapUtils`     | Conditional insert and fallback get for `HashMap` |
| `StrUtils`     | String search helpers + `.to_title_case()` |
| `MemUtils`     | Type reflection: `.type_name()`, `.mem_size()` |
| `DurationUtils`| Pretty formatting of `std::time::Duration` |
| `ConvertUtils` | TryFrom sugar: `.to()`, `.to_result()` |
| `ClampUtils`   | Clamp integers to a range |
| `NumberUtils`  | Simple `.is_even()` / `.is_odd()` |
| `IteratorUtils`| `.find_map_or()` fallback on iterators |
| `IdentityUtils`| Chainable `.tap()` for debug or logging |
| `PanicUtils`   | Exit-friendly unwrapping: `.unwrap_or_exit()` |

### Functions

| Function               | Description                                      |
| ---------------------- | ------------------------------------------------ |
| `Log::log_*()`         | Log with a specific level (`info`, `warn`, etc.) |
| `Log::get_logs()`      | Returns formatted log entries                    |
| `Log::print_logs()`    | Prints all logs to stdout                        |
| `Log::clear()`         | Clears all logs                                  |
| `Log::set_up_logger()` | Sets the active log level filter                 |

## Installation

```toml
# Cargo.toml
[dependencies]
utilz-rs = "0.1"
````

No dependencies. No macros. Just clean, simple, useful sugar.

---

## Usage Examples

### `OptionUtils`

```rust
use uitlz_rs::OptionUtils;

let val = Some("hello");
val.if_some(|s| println!("Got {}", s));

let fallback = val.or_default_with("default");
```

### `ResultUtils`

```rust
use uitlz_rs::ResultUtils;

let res: Result<i32, &str> = Ok(42);
res.if_ok(|v| println!("Success: {v}"));
res.if_err(|e| println!("Error: {e}"));
```

### `BoolUtils`

```rust
use uitlz_rs::BoolUtils;

let mut flag = true;
flag.toggle(); // becomes false

let val = flag.then_val("Yes"); // None
```

### `VecUtils`

```rust
use uitlz_rs::VecUtils;

let mut list = vec![];
list.push_if(10, true);
list.push_if_with(false, || expensive_computation());
```

### `StrUtils`

```rust
use uitlz_rs::StrUtils;

let s = "hello world";
assert!(s.contains_all(["hello", "world"]));
println!("{}", s.to_title_case()); // "Hello world"
```

### `DurationUtils`

```rust
use uitlz_rs::DurationUtils;

let d = Duration::from_secs(3661);
println!("{}", d.pretty()); // "1h 1m 1s"
```

</details>

---

## Full Trait and Method Reference

| Trait           | Method                             | Description                                  |
| --------------- | ---------------------------------- | -------------------------------------------- |
| `EqUtils`       | `eq_to(&self, &T)`                 | Shortcut for `self == other`                 |
|                 | `not_eq_to(&self, &T)`             | Shortcut for `self != other`                 |
| `OptionUtils`   | `or_default_with(self, fallback)`  | Returns value or fallback                    |
|                 | `if_some(self, f)`                 | Calls function if `Some`                     |
| `StrUtils`      | `contains_all(self, iter)`         | Returns `true` if all substrings exist       |
|                 | `contains_any(self, iter)`         | Returns `true` if any substring exists       |
|                 | `to_title_case(self)`              | Capitalizes first letter                     |
| `MemUtils`      | `type_name(&self)`                 | Returns type name                            |
|                 | `mem_size(&self)`                  | Returns memory size in bytes                 |
|                 | `view(&self)`                      | Prints type and size info                    |
| `ConvertUtils`  | `to(self)`                         | Attempts conversion via `TryFrom` (`Option`) |
|                 | `to_or(self, fallback)`            | Uses fallback if conversion fails            |
|                 | `to_result(self)`                  | Conversion result as `Result`                |
| `BoolUtils`     | `not(&self)`                       | Inverts the boolean                          |
|                 | `then_val(&self, val)`             | Returns `Some(val)` if `true`                |
|                 | `if_true(&self, f)`                | Executes `f()` if `true`                     |
|                 | `if_false(&self, f)`               | Executes `f()` if `false`                    |
|                 | `toggle(&mut self)`                | Inverts boolean in-place                     |
| `VecUtils`      | `push_if(&mut self, val, cond)`    | Pushes if condition is true                  |
|                 | `push_if_with(&mut self, cond, f)` | Lazily pushes if condition is true           |
| `MapUtils`      | `get_or(&self, key, fallback)`     | Gets value or fallback if key missing        |
|                 | `insert_if(&mut self, k, v, cond)` | Inserts into map if condition is true        |
| `ResultUtils`   | `if_ok(self, f)`                   | Executes on `Ok`                             |
|                 | `if_err(self, f)`                  | Executes on `Err`                            |
| `DurationUtils` | `pretty(&self)`                    | Formats as `"1h 2m 3s"`                      |
| `IteratorUtils` | `find_map_or(self, f, fallback)`   | Fallback if no `find_map` match              |
| `IdentityUtils` | `tap(self, f)`                     | Taps into chain with a side-effect function  |
| `PanicUtils`    | `unwrap_or_exit(self, msg)`        | Exits if `None` or `Err`                     |
| `ClampUtils`    | `clamp_to(self, min, max)`         | Clamps integer to a range                    |
| `NumberUtils`   | `is_even(&self)`                   | Checks if number is even                     |
|                 | `is_odd(&self)`                    | Checks if number is odd                      |

---

## Philosophy

* 🔧 100% Rust standard library
* 🚫 No dependencies
* ✅ All traits opt-in (use only what you need)
* ✅ Improves clarity without sacrificing performance

---

## License

Licensed under:

* MIT license ([LICENSE](LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

---

## ❤️ Contributing

Issues, discussions, and PRs are welcome.
This crate aims to stay **small, focused, and ergonomic**.
Open to adding new traits if they're genuinely useful and zero-dependency compatible!
