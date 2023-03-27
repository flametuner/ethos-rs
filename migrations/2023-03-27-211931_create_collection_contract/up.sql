-- Your SQL goes here

CREATE TABLE collection_contracts (
    id uuid PRIMARY KEY,
    
    contract_id uuid NOT NULL,

    address varchar(255) NOT NULL,
    fee_recipient varchar(255) NOT NULL,


    collection_id uuid NOT NULL REFERENCES collections(id),
    network_id uuid NOT NULL REFERENCES networks(id)
);

