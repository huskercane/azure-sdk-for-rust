/*
prints information about network addresses used for each Azure Service

$ cargo run --release --example list_ip_configuration <resource_group>
$

*/

use azure_identity::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_group_name = std::env::args().nth(1).expect("please specify resource group name");
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    let network_client = azure_mgmt_network::Client::builder(credential).build()?;

    let nics = network_client.network_interfaces_client().list(resource_group_name, subscription_id).await.unwrap().value;

    for entry in nics {
        if let Some(props) = entry.properties {
            let ip_configs = props.ip_configurations;
            for ip_config in ip_configs {
                if let Some(ip_config_props) = ip_config.properties {
                    if let Some(private_ip_address) = ip_config_props.private_ip_address {
                        println!("{:?}",  private_ip_address);
                    }
                }
            }
        }
    }

    Ok(())
}
