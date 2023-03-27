use async_graphql::SimpleObject;
use diesel::prelude::*;
use ethers::types::Address;
use ethers::utils::to_checksum;

use diesel::{r2d2::ConnectionManager, Insertable, PgConnection, Queryable, RunQueryDsl};
use ethers::types::Signature;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{database::ConnectionPool, errors::StoreError, schema::wallets};

use super::profile::ProfileService;

#[derive(Debug, Queryable, SimpleObject, Serialize, Deserialize)]
pub struct Wallet {
    id: Uuid,
    address: String,
    nonce: Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    pub profile_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = wallets)]
struct NewWallet {
    address: String,
    nonce: Uuid,
    profile_id: Uuid,
}

pub struct WalletService {
    pool: ConnectionPool,
    profile_service: Arc<ProfileService>,
}

impl WalletService {
    pub fn new(
        pool: Pool<ConnectionManager<PgConnection>>,
        profile_service: Arc<ProfileService>,
    ) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
            profile_service,
        }
    }

    pub fn get_wallet(&self, addr: &Address) -> Result<Wallet, StoreError> {
        use crate::schema::wallets::dsl::*;
        let mut conn = self.pool.get()?;

        let mut wallet = wallets
            .filter(address.eq(to_full_addr(&addr)))
            .limit(1)
            .load::<Wallet>(&mut *conn)?;
        if let Some(wallet) = wallet.pop() {
            Ok(wallet)
        } else {
            Err(StoreError::WalletNotFound(to_full_addr(&addr)))
        }
    }
    pub fn upsert_wallet(&self, addr: Address) -> Result<Wallet, StoreError> {
        use crate::schema::wallets::dsl::*;

        if let Ok(wallet) = self.get_wallet(&addr) {
            return Ok(wallet);
        }

        let profile = self.profile_service.new_profile()?;
        let mut conn = self.pool.get()?;
        let new_wallet = NewWallet {
            address: to_full_addr(&addr),
            nonce: Uuid::new_v4(),
            profile_id: profile.id,
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
            .filter(address.eq(to_full_addr(&addr)))
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

        let wallet = self.get_wallet(&addr)?;
        let nonce = wallet.nonce.to_string();

        let message = create_message(&addr, &nonce);
        println!("{}", message);
        // check if the nonce of the signature is the same of the database
        signature.verify(message, addr)?;
        // update the nonce
        // return wallet
        Ok(self.update_nonce(addr)?)
    }
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
