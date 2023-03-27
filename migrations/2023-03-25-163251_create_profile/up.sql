-- Your SQL goes here
CREATE TABLE profiles (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  name VARCHAR(255),
  email VARCHAR(255),
  wallet_id uuid REFERENCES wallets(id) NOT NULL
);
