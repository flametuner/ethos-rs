use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, Insertable, PgConnection, Queryable, RunQueryDsl};
use ethabi::Address;
use ethers::types::Signature;
use ethers::types::U256;
use r2d2::Pool;
use std::str::FromStr;
use uuid::Uuid;

use crate::{database::ConnectionPool, errors::StoreError, schema::wallets};

#[derive(Debug, Queryable, SimpleObject)]
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

    pub fn upsert_wallet(&self, addr: Address) -> Result<Wallet, StoreError> {
        use crate::schema::wallets::dsl::*;

        let mut conn = self.pool.get()?;

        let mut wallet = wallets
            .filter(address.eq(addr.to_string()))
            .limit(1)
            .load::<Wallet>(&mut *conn)?;
        if let Some(wallet) = wallet.pop() {
            return Ok(wallet);
        }
        let new_wallet = NewWallet {
            address: addr.to_string(),
            nonce: Uuid::new_v4(),
        };
        Ok(diesel::insert_into(wallets)
            .values(&new_wallet)
            .get_result::<Wallet>(&mut conn)?)
    }

    pub async fn login(&self, addr: Address, signature: String) -> Result<Wallet, StoreError> {
        let signature = Signature::from_str(&signature)?;
        let nonce = "";
        signature.verify(create_message(&addr.to_string(), nonce), addr)?;
        // verify signature
        // retrive the nonce from the dattabase
        // check if the nonce of the signature is the same of the database
        // update the nonce
        // return the wallet
        todo!()
    }
}

fn create_message(address: &str, nonce: &str) -> String {
    format!(
        "Welcome\n
Click to sign in and accept the Terms of Service
This request will not trigger a blockchain transaction or cost any gas fees.
Your authentication status will reset after 24 hours.\n
Wallet address:
{}\n
Nonce:
{}",
        address, nonce
    )
}
