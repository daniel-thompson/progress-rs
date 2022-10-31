A small library of extension traits to control and monitor the progress
of iterators.

# Examples

```rust
use progress::*;

for n in (0..27)
    .rate_limit(std::time::Duration::from_millis(10))
    .show_percent()
{
    // do something interesting...
}
```
