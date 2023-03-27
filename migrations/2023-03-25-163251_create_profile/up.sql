-- Your SQL goes here
CREATE TABLE profiles (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  name VARCHAR(255),
  email VARCHAR(255)
);

ALTER TABLE wallets ADD profile_id uuid NOT NULL;

ALTER TABLE wallets 
ADD FOREIGN KEY(profile_id)
REFERENCES wallets(id);
