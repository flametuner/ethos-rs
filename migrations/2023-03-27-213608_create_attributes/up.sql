-- Your SQL goes here
CREATE TYPE display_type AS ENUM ('number', 'boost_percentage', 'boost_number');
CREATE TABLE nft_attributes (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  trait_type VARCHAR(255),
  value VARCHAR(255),
  max_value VARCHAR(255),
  display_type display_type 
);
