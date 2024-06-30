# cycle
An MVP of E2EE functions.

## Compile
Use `cargo` tool to build the project.
* Debug mode:
```bash
> cargo build
```
* Release mode:
```bash
> cargo build --release
```

## Test
Currently there are few test cases, to run them
* Without `println` information inside testcase:
```bash
> cargo test
```
* With `println` information inside testcase:
```bash
> cargo test -- --nocapture
```