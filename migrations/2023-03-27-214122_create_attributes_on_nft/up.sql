-- Your SQL goes here
CREATE TABLE attributes_on_nfts (
  nft_id uuid REFERENCES nfts(id),
  attribute_id uuid REFERENCES nft_attributes(id),

  PRIMARY KEY (nft_id, attribute_id)
);
