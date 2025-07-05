use async_move_trait::async_move_trait;
use rstest::rstest;
use tokio;

#[async_move_trait]
fn expr_body(x: &i32) -> i32 {
    let capture = *x;
    async move {};
    capture
}
#[rstest]
#[tokio::test]
async fn test_expr_body() {
    let x = 5;
    let fut = expr_body(&x);
    let result = fut.await;
    println!("Result: {}", result);
    assert_eq!(result, 5);
}
