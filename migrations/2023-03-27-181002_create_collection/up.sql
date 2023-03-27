-- Your SQL goes here

CREATE TABLE collections (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name varchar(255) NOT NULL,
  description text,
  image varchar(255),
  external_link varchar(255),
  seller_fee_basis_points integer,
  project_id uuid NOT NULL REFERENCES projects(id),

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
