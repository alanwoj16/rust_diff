# rust-diff
A diff library written in rust

[**Documentation**](https://eecs395rust.github.io/rust-diff/)

## Example Usage

### Calculate the diff
```Rust 
use diff::{diff, patch, pretty_print};
use std::io::stdout;

let from = vec!["this", "is", "an", "example"];
let to = vec!["this", "is", "another", "example"];

// generate the edit script
let changes = diff(&from, &to);
```
Each edit in the diff is represented by a [DiffItem](https://eecs395rust.github.io/rust-diff/diff/enum.DiffItem.html)

### Print the diff (gnu diff format)
```Rust
for edit in &changes {
    print!("{}", *edit);
}
```

### Apply a change specified by a DiffItem
```Rust
// apply the first edit in the script
let patched = patch(&from, &changes[0]);
```

### print out a colored representation of a single change

```Rust
pretty_print(stdout(), &from, &changes[0]);
```
