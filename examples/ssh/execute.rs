use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};

#[tokio::main]
async fn main() -> Result<(), async_ssh2_tokio::Error> {
    // if you want to use key auth, then use following:
    // AuthMethod::with_key_file("key_file_name", Some("passphrase"));
    // or
    // AuthMethod::with_key_file("key_file_name", None);
    // or
    // AuthMethod::with_key(key: &str, passphrase: Option<&str>)
    let auth_method = AuthMethod::with_key_file("/Users/liuhu016/.ssh/id_rsa", None);
    let mut client = Client::connect(
        ("10.11.1.3", 22),
        "root",
        auth_method,
        ServerCheckMethod::NoCheck,
    )
    .await?;

    let result = client.execute("echo Hello SSH").await?;
    print!("{}", result.stdout);

    Ok(())
}
