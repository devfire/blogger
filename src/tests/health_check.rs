// if a test fails, we panic. No need to propagate errors
fn spawn_app() {
    let server = blogger::run().expect("Failed to bind to specified address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // except we don't need it, so _
    let _ = tokio::spawn(server);
}