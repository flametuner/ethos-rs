use async_graphql::SimpleObject;
use diesel::prelude::*;
use ethers::types::Address;
use ethers::utils::to_checksum;

use diesel::{Insertable, Queryable, RunQueryDsl};
use ethers::types::Signature;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::{database::ConnectionPool, errors::EthosError, schema::wallets};

#[derive(Debug, Queryable, SimpleObject, Serialize, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = wallets)]
pub struct Wallet {
    pub id: Uuid,
    pub address: String,
    nonce: Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = wallets)]
struct NewWallet {
    address: String,
    nonce: Uuid,
}

pub struct WalletService {
    pool: ConnectionPool,
}

impl WalletService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    pub fn get_wallet(&self, addr: &Address) -> Result<Wallet, EthosError> {
        use crate::schema::wallets::dsl::*;
        let mut conn = self.pool.get()?;

        let wallet = wallets
            .filter(address.eq(to_full_addr(&addr)))
            .first::<Wallet>(&mut conn)?;
        Ok(wallet)
    }

    pub fn upsert_wallet(&self, addr: Address) -> Result<Wallet, EthosError> {
        use crate::schema::wallets::dsl::*;

        let wallet = match self.get_wallet(&addr) {
            Ok(wallet) => wallet,
            Err(_) => {
                let mut conn = self.pool.get()?;
                let new_wallet = NewWallet {
                    address: to_full_addr(&addr),
                    nonce: Uuid::new_v4(),
                };

                diesel::insert_into(wallets)
                    .values(&new_wallet)
                    .get_result::<Wallet>(&mut conn)?
            }
        };
        Ok(wallet)
    }

    fn update_nonce(&self, addr: Address) -> Result<Wallet, EthosError> {
        use crate::schema::wallets::dsl::*;

        let mut conn = self.pool.get()?;

        let new_nonce = Uuid::new_v4();
        let wallet = diesel::update(wallets)
            .filter(address.eq(to_full_addr(&addr)))
            .set(nonce.eq(new_nonce))
            .get_result::<Wallet>(&mut conn)?;

        Ok(wallet)
    }

    pub async fn login(&self, addr: Address, signature: String) -> Result<Wallet, EthosError> {
        // verify signature
        let wallet = self.get_wallet(&addr)?;
        verify_signature(addr, &wallet.nonce.to_string(), signature).await?;

        // update the nonce
        // return wallet
        Ok(self.update_nonce(addr)?)
    }
}

async fn verify_signature(
    addr: Address,
    nonce: &String,
    signature: String,
) -> Result<(), EthosError> {
    // verify signature
    let signature = Signature::from_str(&signature)?;
    // retrive the nonce from the dattabase

    let message = create_message(&addr, &nonce);
    // check if the nonce of the signature is the same of the database
    signature.verify(message, addr)?;
    Ok(())
}

fn create_message(address: &Address, nonce: &str) -> String {
    format!(
        "Welcome\n\n\
            Click to sign in and accept the Terms of Service\n\
            This request will not trigger a blockchain transaction or cost any gas fees.\n\
            Your authentication status will reset after 24 hours.\n\n\
            Wallet address:\n\
            {}\n\n\
            Nonce:\n\
            {}",
        to_full_addr(address),
        nonce
    )
}

fn to_full_addr(addr: &Address) -> String {
    to_checksum(addr, None)
}

#[cfg(test)]
mod tests {
    use crate::services::wallet::verify_signature;
    use std::fmt::Debug;

    use dotenvy::dotenv;
    use ethers::{
        signers::{LocalWallet, Signer},
        types::Address,
        utils::to_checksum,
    };
    use fixed_hash::rand::thread_rng;
    use uuid::Uuid;

    use crate::{
        database::{create_connection_pool, ConnectionPool},
        errors::EthosError,
        services::wallet::create_message,
    };

    use super::WalletService;

    fn execute_transaction<F, T, E>(function: F)
    where
        F: FnOnce(ConnectionPool) -> Result<T, E>,
        E: Debug,
    {
        dotenv().ok();
        let database_connection = create_connection_pool();
        let pool = ConnectionPool::new(database_connection);
        function(pool).unwrap();
    }

    #[test]
    fn test_wallet_creation() {
        execute_transaction::<_, _, EthosError>(|pool| {
            let wallet_service = WalletService::new(pool);
            let addr = Address::random();
            let wallet = wallet_service.upsert_wallet(addr)?;
            assert_eq!(to_checksum(&addr, None), wallet.address);

            let returned_wallet = wallet_service.get_wallet(&addr)?;
            assert_eq!(wallet, returned_wallet);

            Ok(())
        })
    }

    #[tokio::test]
    async fn test_verify_signature() {
        let signer = LocalWallet::new(&mut thread_rng());
        let nonce = Uuid::new_v4().to_string();
        let message = create_message(&signer.address(), &nonce);
        let signature = signer.sign_message(message).await.unwrap().to_string();
        verify_signature(signer.address(), &nonce, signature)
            .await
            .expect("It didn't verified the signature correctly");
    }
}
