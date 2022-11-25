// // define an Ip:Port combo from https://doc.rust-lang.org/std/net/enum.IpAddr.html
// struct URL {
//     Protocol: String,
//     Ip: IpAddr::V4,
//     Port: i32,
// }

// if a test fails, we panic. No need to propagate errors
fn spawn_app() -> String {
  
    let listener = TcpListener::bind("127.0.0.1:0")?;

    // retrieve the port chosen by the OS
    let port = listener.local_addr().port()?;

    let server = blogger::run(listener)?;
    
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // except we don't need it, so _
    let _ = tokio::spawn(server);

    // return the enum back to the caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app();

    // issue requests against the app
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

        // make sure we get 200 back
        assert!(response.status().is_success());

        // make sure there's no body
        assert_eq!(Some(0), response.content_length());
}