use std::collections::HashMap;

use dotenvy::dotenv;
use ethos_rs::{
    database::create_connection_pool,
    services::{
        nft::{Network, NftService},
        project::ProjectService,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_connection = create_connection_pool();

    let project_service = ProjectService::new(database_connection.clone());
    let nft_service = NftService::new(database_connection.clone());
    // create project
    let project_name = "Taipe Experience";
    let project = match project_service.get_project_by_name(project_name).ok() {
        Some(project) => project,
        None => project_service.create_project(project_name, None)?,
    };

    println!("{:?}", project);
    // create collection
    let collection_name = "Taipe Experience";
    let collection = match nft_service
        .get_collection_by_name(&project, collection_name)
        .ok()
    {
        Some(collection) => collection,
        None => nft_service.create_collection(&project, collection_name.clone(), None)?,
    };

    println!("{:?}", collection);

    // create network
    let mut network_contract = HashMap::new();
    network_contract.insert(
        5,
        (
            "0x94E45dCE34b3030dEDdB72C2D41f20444ef5D4CE",
            "0xC40e55c684B63Ffc3c9127A1156c9d84c62A69ab",
        ),
    );
    network_contract.insert(
        80001,
        (
            "0xe2118B9EBC0217eEe5D56b2D11198363D66358AE",
            "0x9c5298016D8157aF0837906317E378ce09bc4135",
        ),
    );

    let networks = network_contract
        .into_iter()
        .map(|(network_id, (address, fee_recipient))| {
            let network = match nft_service.get_network_by_id(network_id) {
                Ok(network) => network,
                Err(_) => nft_service.create_network(network_id).unwrap(),
            };

            println!("{:?}", network);
            (network, address, fee_recipient)
        })
        .collect::<Vec<(Network, &str, &str)>>();

    // create collection contracts

    networks
        .into_iter()
        .for_each(|(network, contract_address, fee_recipient)| {
            let collection_contract =
                match nft_service.get_collection_contract_by_address(&network, contract_address) {
                    Ok(collection_contract) => collection_contract,
                    Err(_) => nft_service
                        .create_collection_contract(
                            &collection,
                            &network,
                            contract_address,
                            fee_recipient,
                        )
                        .unwrap(),
                };

            println!("{:?}", collection_contract);
        });
    Ok(())
}
