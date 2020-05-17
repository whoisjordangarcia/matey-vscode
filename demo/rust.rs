
fn main() {
    #[cfg(feature = "diesel")]
    {
        dotenv().ok();
    }

    let port = iis::get_port();

    let listen_on = format!("127.0.0.1:{}", port);

    println!("Listening on {}", listen_on);

    let mut builder = RouterBuilder::new();

    // Use raw strings so you don't need to escape patterns.
    builder.get(r"/", hello_handler);

    #[cfg(feature = "tiberius")] builder.post(r"/createdb", create_db_handler);

    builder.post(r"/api/users/login", authentication_handler);
    builder.post(r"/api/users", registration_handler);
    builder.get(r"/api/user", get_current_user_handler);
}