use api_helper::{
	auth::{ApiAuth, AuthRateLimitCtx},
	util::as_auth_expired,
};
use proto::claims::Claims;
use tivet_claims::ClaimsDecode;
use tivet_operation::prelude::*;

/// Information derived from the authentication middleware.
pub struct Auth {
	config: tivet_config::Config,
	claims: Option<Claims>,
}

#[async_trait]
impl ApiAuth for Auth {
	async fn new(
		config: tivet_config::Config,
		api_token: Option<String>,
		rate_limit_ctx: AuthRateLimitCtx<'_>,
	) -> GlobalResult<Auth> {
		Self::rate_limit(&config, rate_limit_ctx).await?;

		Ok(Auth {
			config: config.clone(),
			claims: if let Some(api_token) = api_token {
				Some(as_auth_expired(tivet_claims::decode(
					&config.server()?.jwt.public,
					&api_token,
				)?)?)
			} else {
				None
			},
		})
	}

	async fn rate_limit(
		config: &tivet_config::Config,
		_rate_limit_ctx: AuthRateLimitCtx<'_>,
	) -> GlobalResult<()> {
		Ok(())
	}
}

impl Auth {
	pub fn claims(&self) -> GlobalResult<&Claims> {
		self.claims
			.as_ref()
			.ok_or_else(|| err_code!(API_UNAUTHORIZED, reason = "No bearer token provided."))
	}

	pub fn server(&self) -> GlobalResult<tivet_claims::ent::ProvisionedServer> {
		self.claims()?.as_provisioned_server()
	}
}
