# async_move_trait
An Async move trait macro that transforms the function into `impl Future`.

This crate provides the `#[async_move_trait]` procedural macro attribute, which transforms a function return type becomes `impl Future`.

## Example

```rs
use async_move_trait::async_move_trait;

#[async_move_trait]
fn (r: &i32) -> i32 {
  let capture = *r;

  async move{};
   
  capture
}
```
The above expands to:
```rs
fn (r: &i32) -> impl Future<Output = i32> {
  let capture = *r;

  async move {
    capture
  }
}
```
