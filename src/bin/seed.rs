use std::collections::HashMap;

use dotenvy::dotenv;
use ethos_rs::{
    database::create_connection_pool,
    services::{
        nft::{
            AttributesOnNft, CollectionContract, Network, NewNft, Nft, NftAttribute, NftService,
        },
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

    let collection_contracts: Vec<CollectionContract> = networks
        .into_iter()
        .map(|(network, contract_address, fee_recipient)| {
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
            collection_contract
        })
        .collect();

    // create nfts
    let nfts = match nft_service.get_nfts_by_collection_id(collection.id) {
        Ok(nfts) if nfts.len() > 0 => nfts,
        _ => {
            let image_url = "https://assets.taipe.xyz/nft";
            let animation_url = "https://singulari3-turborepo-backoffice.vercel.app/collection";
            let description = "🪩 🦎 Langoo! 🦎 🪩 ∞ ∞ There are 12.000 Langoos! around. They are Brazilian Mystic Creatures ready to steal the spotlight.Langoo! is a metaphor · a lifestyle.Some of them live in the jungle · Some in the big cities · But they really like the coast · They've evolved from there to the rest of the world. They're here way before us. They represent a concept long forgot; ¨ancestry¨.But the flame is still alive ... Each category has its own lore. Visit https://festadotaipe.xyz/onboarding for more info.";

            let nfts_data: Vec<NewNft> = (1..=12000)
                .map(|i| {
                    let extension = if i <= 25 { "gif " } else { "png" };
                    let contract_network = if i <= 25 {
                        collection_contracts.get(0).unwrap()
                    } else {
                        collection_contracts.get(1).unwrap()
                    };
                    let nft_data = NewNft {
                        nft_id: i,
                        name: format!("Langoo! {}", i),
                        image: format!("{}/{}.{}", image_url, i, extension),
                        description: description.to_string(),
                        external_url: format!("https://taipe.xyz/nft/{}", i),
                        animation_url: format!(
                            "{}/{}/nft/{}",
                            animation_url,
                            collection.id.to_string(),
                            i
                        ),
                        collection_id: collection.id,
                        network_contract_id: contract_network.id,
                    };
                    nft_data
                })
                .collect();

            let nfts = nfts_data
                .chunks(1000)
                .into_iter()
                .map(|chunk| {
                    let nfts = nft_service.create_nfts(chunk.to_vec()).unwrap();
                    nfts
                })
                .flatten()
                .collect::<Vec<Nft>>();
            nfts
        }
    };
    // check if already exists nfts before creating

    println!("{} nfts", nfts.len());
    let first_nft = nfts.get(0).unwrap();
    println!("First: {:?}", first_nft);

    println!("Last: {:?}", nfts.get(nfts.len() - 1).unwrap());

    // create tier attributes

    let attributes = match nft_service.get_attributes_from_type("Tier") {
        Ok(attr) if attr.len() > 0 => attr,
        Ok(_) | Err(_) => {
            let attributes: Vec<NftAttribute> = (1..=3)
                .map(|i| {
                    let tier_attribute = nft_service.create_attribute(
                        Some("Tier"),
                        Some(format!("{}", i)),
                        None,
                        None,
                    );

                    tier_attribute.unwrap()
                })
                .collect();

            attributes
        }
    };

    println!("{:?}", attributes);
    // create nft attribute relations

    let attributes_on_nft = match nft_service.get_nft_attributes(first_nft.id) {
        Ok(attr) if attr.len() > 0 => attr,
        Ok(_) | Err(_) => {
            let attributes_on_nft: Vec<AttributesOnNft> = nfts
                .iter()
                .map(|nft| {
                    let attribute_id = if nft.nft_id <= 25 {
                        attributes.get(0).unwrap().id
                    } else if nft.nft_id <= 500 {
                        attributes.get(1).unwrap().id
                    } else {
                        attributes.get(2).unwrap().id
                    };
                    let attr = nft_service.create_attribute_nft_relation(nft.id, attribute_id);

                    attr.unwrap()
                })
                .collect();
            attributes_on_nft
        }
    };

    println!("{:?}", attributes_on_nft.get(0).unwrap());

    Ok(())
}
