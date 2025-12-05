//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`
// It also spares you from having to specify the `#[test` attribute

// cmd to run `cargo watch -x check -x test -x run`
#[tokio::test]
async fn health_check_works(){
    //Arrange
    spawn_app();

    //We need to bring in `reqwest`
    //to perform HTTP requess against our application.

    let client = reqwest::Client::new();

    //Act
    let _response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(_response.status().is_success());
    assert_eq!(Some(0), _response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}