// if a test fails, we panic. No need to propagate errors
fn spawn_app() {
    let server = blogger::run().expect("Failed to bind to specified address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // except we don't need it, so _
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_test() {
    spawn_app();

    // issue requests against the app
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

        // make sure we get 200 back
        assert!(response.status().is_success());

        // make sure there's no body
        assert_eq!(Some(0)), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    
}