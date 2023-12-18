use aws_sdk_ec2 as ec2;

#[::tokio::main]
async fn main() -> Result<(), ec2::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ec2::Client::new(&config);

    // ... make some calls with the client
    let vpcs = client.describe_vpcs().send().await?;

    println!(
        "{:?}",
        vpcs.vpcs()[0]
            .vpc_id()
            .as_deref()
            .unwrap_or_default()
            .as_bytes()
    );
    println!("{:?}", Some("vpc".as_bytes()));
    Ok(())
}
