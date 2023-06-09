use std::sync::Arc;

use async_graphql::SimpleObject;

use crate::{errors::EthosError, jwt::JwtAuthentication};
use ethers::types::Address;

use super::wallet::{Wallet, WalletService};

#[derive(Debug, SimpleObject)]
pub struct LoginResponse {
    token: String,
    wallet: Wallet,
}

pub struct AuthService {
    wallet_service: Arc<WalletService>,
    jwt_auth: JwtAuthentication,
}
impl AuthService {
    pub fn new(wallet_service: Arc<WalletService>) -> Self {
        AuthService {
            wallet_service,
            jwt_auth: JwtAuthentication::new(),
        }
    }
    pub async fn login(
        &self,
        addr: Address,
        signature: String,
    ) -> Result<LoginResponse, EthosError> {
        let wallet = self.wallet_service.get_wallet(&addr)?;
        let wallet = self
            .wallet_service
            .verify_and_update_nonce(&wallet, signature)
            .await?;
        let token = self.jwt_auth.create_token(&wallet)?;
        Ok(LoginResponse { token, wallet })
    }

    pub async fn validate(&self, token: &str) -> Result<Wallet, EthosError> {
        Ok(self.jwt_auth.validate(token)?)
    }
}
