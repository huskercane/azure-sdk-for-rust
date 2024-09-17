/*
prints information about network addresses used for each Azure Service

$ cargo run --release --example list_network_configuration <resource_group> <vm_name>
$

*/

use azure_identity::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_group_name = std::env::args().nth(1).expect("please specify resource group name");
    let vm_name = std::env::args().nth(2).expect("please specify vm group name");

    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    let compute_client = azure_mgmt_compute::Client::builder(credential.clone()).build()?;

    // Get the VM
    let vm = compute_client.virtual_machines_client().get(resource_group_name, vm_name, subscription_id).await?;

    let interface_name = vm.properties.unwrap().network_profile.unwrap().network_interfaces[0].clone().id.unwrap();

    let last_part = interface_name.split('/').last().unwrap_or("");
    println!("Interface Name is: {}", last_part);

    Ok(())
}