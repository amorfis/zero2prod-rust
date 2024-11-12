use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let port = spawn_app().expect("Failed to spawn our app.");
    // We need to bring in `reqwest` 
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() -> Result<u16, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to run server.");
    tokio::spawn(server);

    Ok(port)
}
