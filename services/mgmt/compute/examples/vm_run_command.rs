/*
cargo run --package azure_mgmt_compute --example vm_run_command
*/

use azure_identity::AzureCliCredential;
use azure_mgmt_compute::models::RunCommandInput;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    let client = azure_mgmt_compute::Client::builder(credential).build()?;
    let resource_group_name = "test-resource-group";
    let vm_name = "test-vm";
    let ps_command = "Get-Date";

     let command = RunCommandInput {
        // https://learn.microsoft.com/en-us/azure/virtual-machines/windows/run-command
        // https://learn.microsoft.com/en-us/rest/api/compute/virtual-machines/run-command?view=rest-compute-2024-07-01&tabs=HTTP
        command_id: "RunPowerShellScript".to_string(),
        script: vec![ps_command.to_string()],
        parameters: vec![],
    };
    let response = client
        .virtual_machines_client()
        .run_command(resource_group_name, vm_name, command, subscription_id)
        .await?;

    response.value.iter().for_each(|x| {
        log::debug!("Response: {:#?}", x);
        let message = x.message.clone().unwrap_or_else(|| "".to_string());
        println!("Message is: {message}");
    });

    Ok(())
}
