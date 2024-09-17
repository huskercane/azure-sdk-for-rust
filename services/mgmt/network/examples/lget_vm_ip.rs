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
    let vm_name = std::env::args().nth(2).expect("please specify vm group name");

    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    let network_client = azure_mgmt_network::Client::builder(credential).build()?;
    let compute_client = azure_mgmt_compute::Client::builder(credential.clone()).build()?;

    // Get the VM
    let vm = compute_client.virtual_machines_client().get(resource_group_name, vm_name, subscription_id).await?;

    // Get the VM
    let properties = vm.properties.unwrap();
    let vec1 = properties.network_profile.unwrap().network_interface_configurations;
    let interface_name = vec1.get(0).unwrap().name.clone();
    let nic = network_client.network_interfaces_client().get(resource_group_name, interface_name, subscription_id).await?;

    let vec = nic.properties.unwrap().ip_configurations;
    let x1 = vec.get(0).unwrap().clone();
    let x = x1.properties.unwrap().private_ip_address.unwrap();
    println!("{:?}", x);

    Ok(())
}
