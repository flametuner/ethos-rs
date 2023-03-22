-- Your SQL goes here

CREATE TABLE wallets (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  address VARCHAR(255) NOT NULL UNIQUE,
  nonce uuid NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
