-- Your SQL goes here

CREATE TABLE nfts (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  nft_id INTEGER NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  minted_at TIMESTAMP,
  image VARCHAR(255) NOT NULL,
  external_url VARCHAR(255) NOT NULL,
  
  owner_id uuid REFERENCES wallets(id),
  collection_id uuid NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
  network_contract_id uuid NOT NULL REFERENCES collections(id) ON DELETE CASCADE
);
