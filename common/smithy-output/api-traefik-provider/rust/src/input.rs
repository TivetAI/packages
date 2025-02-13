// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`TraefikConfigInput`](crate::input::TraefikConfigInput)
pub mod traefik_config_input {
	/// A builder for [`TraefikConfigInput`](crate::input::TraefikConfigInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) token: std::option::Option<std::string::String>,
		pub(crate) pool: std::option::Option<std::string::String>,
		pub(crate) region: std::option::Option<std::string::String>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn token(mut self, input: impl Into<std::string::String>) -> Self {
			self.token = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_token(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.token = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn pool(mut self, input: impl Into<std::string::String>) -> Self {
			self.pool = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_pool(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.pool = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
			self.region = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.region = input;
			self
		}
		/// Consumes the builder and constructs a [`TraefikConfigInput`](crate::input::TraefikConfigInput)
		pub fn build(
			self,
		) -> std::result::Result<
			crate::input::TraefikConfigInput,
			aws_smithy_http::operation::BuildError,
		> {
			Ok(crate::input::TraefikConfigInput {
				token: self.token,
				pool: self.pool,
				region: self.region,
			})
		}
	}
}
#[doc(hidden)]
pub type TraefikConfigInputOperationOutputAlias = crate::operation::TraefikConfig;
#[doc(hidden)]
pub type TraefikConfigInputOperationRetryAlias = ();
impl TraefikConfigInput {
	/// Consumes the builder and constructs an Operation<[`TraefikConfig`](crate::operation::TraefikConfig)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::TraefikConfig, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::TraefikConfigInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/traefik/config").expect("formatting should succeed");
				Ok(())
			}
			fn uri_query(
				_input: &crate::input::TraefikConfigInput,
				mut output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				let mut query = aws_smithy_http::query::Writer::new(&mut output);
				if let Some(inner_1) = &_input.token {
					query.push_kv("token", &aws_smithy_http::query::fmt_string(inner_1));
				}
				if let Some(inner_2) = &_input.pool {
					query.push_kv("pool", &aws_smithy_http::query::fmt_string(inner_2));
				}
				if let Some(inner_3) = &_input.region {
					query.push_kv("region", &aws_smithy_http::query::fmt_string(inner_3));
				}
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::TraefikConfigInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				uri_query(input, &mut _uri)?;
				Ok(builder.method("GET").uri(_uri))
			}
			let mut builder = update_http_builder(&self, _config, http::request::Builder::new())?;
			let mut builder = if let Some(auth) = &_config.auth {
				builder.header(http::header::AUTHORIZATION, auth.clone())
			} else {
				builder
			};
			builder
		};
		let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
		#[allow(clippy::useless_conversion)]
		let body = aws_smithy_http::body::SdkBody::from("");
		let request = request.body(body).expect("should be valid request");
		let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
		request
			.properties_mut()
			.insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
		let op = aws_smithy_http::operation::Operation::new(
			request,
			crate::operation::TraefikConfig::new(),
		)
		.with_metadata(aws_smithy_http::operation::Metadata::new(
			"TraefikConfig",
			"RouteService",
		));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`TraefikConfigInput`](crate::input::TraefikConfigInput)
	pub fn builder() -> crate::input::traefik_config_input::Builder {
		crate::input::traefik_config_input::Builder::default()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TraefikConfigInput {
	#[allow(missing_docs)] // documentation missing in model
	pub token: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub pool: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub region: std::option::Option<std::string::String>,
}
impl TraefikConfigInput {
	#[allow(missing_docs)] // documentation missing in model
	pub fn token(&self) -> std::option::Option<&str> {
		self.token.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn pool(&self) -> std::option::Option<&str> {
		self.pool.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn region(&self) -> std::option::Option<&str> {
		self.region.as_deref()
	}
}
impl std::fmt::Debug for TraefikConfigInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("TraefikConfigInput");
		formatter.field("token", &self.token);
		formatter.field("pool", &self.pool);
		formatter.field("region", &self.region);
		formatter.finish()
	}
}
