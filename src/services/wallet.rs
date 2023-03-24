use async_graphql::SimpleObject;
use diesel::prelude::*;
use ethers::types::Address;

use diesel::{r2d2::ConnectionManager, Insertable, PgConnection, Queryable, RunQueryDsl};
use ethers::types::Signature;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::{database::ConnectionPool, errors::StoreError, schema::wallets};

#[derive(Debug, Queryable, SimpleObject, Serialize, Deserialize)]
pub struct Wallet {
    id: Uuid,
    address: String,
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
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn get_wallet(&self, addr: Address) -> Result<Wallet, StoreError> {
        use crate::schema::wallets::dsl::*;
        let mut conn = self.pool.get()?;

        let mut wallet = wallets
            .filter(address.eq(addr.to_string()))
            .limit(1)
            .load::<Wallet>(&mut *conn)?;
        if let Some(wallet) = wallet.pop() {
            Ok(wallet)
        } else {
            Err(StoreError::WalletNotFound(addr.to_string()))
        }
    }
    pub fn upsert_wallet(&self, addr: Address) -> Result<Wallet, StoreError> {
        use crate::schema::wallets::dsl::*;

        if let Ok(wallet) = self.get_wallet(addr) {
            return Ok(wallet);
        }

        let mut conn = self.pool.get()?;
        let new_wallet = NewWallet {
            address: addr.to_string(),
            nonce: Uuid::new_v4(),
        };
        Ok(diesel::insert_into(wallets)
            .values(&new_wallet)
            .get_result::<Wallet>(&mut conn)?)
    }

    fn update_nonce(&self, addr: Address) -> Result<Wallet, StoreError> {
        use crate::schema::wallets::dsl::*;

        let mut conn = self.pool.get()?;

        let new_nonce = Uuid::new_v4();
        Ok(diesel::update(wallets)
            .filter(address.eq(addr.to_string()))
            .set(nonce.eq(new_nonce))
            .get_result::<Wallet>(&mut conn)?)
    }

    pub async fn verify_signature(
        &self,
        addr: Address,
        signature: String,
    ) -> Result<Wallet, StoreError> {
        // verify signature
        let signature = Signature::from_str(&signature)?;
        // retrive the nonce from the dattabase
        let wallet = self.get_wallet(addr)?;
        let nonce = wallet.nonce.to_string();

        // check if the nonce of the signature is the same of the database
        signature.verify(create_message(&addr.to_string(), &nonce), addr)?;
        // update the nonce
        // return wallet
        Ok(self.update_nonce(addr)?)
    }
}

fn create_message(address: &str, nonce: &str) -> String {
    format!(
        "Welcome\n\n\
            Click to sign in and accept the Terms of Service\n\
            This request will not trigger a blockchain transaction or cost any gas fees.\n\
            Your authentication status will reset after 24 hours.\n\n\
            Wallet address:\n\
            {}\n\n\
            Nonce:\n\
            {}",
        address, nonce
    )
}
