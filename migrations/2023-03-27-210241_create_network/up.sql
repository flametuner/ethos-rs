-- Your SQL goes here

CREATE TABLE networks (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  chain_id integer NOT NULL
);
