use async_graphql::SimpleObject;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, Insertable, PgConnection, Queryable, RunQueryDsl};
use ethabi::Address;
use r2d2::Pool;
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
            .load::<Wallet>(&mut *conn)
            .map_err(|_| StoreError::LoadError)?;
        if let Some(wallet) = wallet.pop() {
            return Ok(wallet);
        }
        let new_wallet = NewWallet {
            address: addr.to_string(),
            nonce: Uuid::new_v4(),
        };
        diesel::insert_into(wallets)
            .values(&new_wallet)
            .get_result::<Wallet>(&mut conn)
            .map_err(|_e| {
                println!("Failed to create wallet: {:?}", _e);
                StoreError::FailedToCreate
            })
    }
}
