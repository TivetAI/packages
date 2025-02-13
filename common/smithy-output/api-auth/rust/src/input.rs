// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`CompleteEmailVerificationInput`](crate::input::CompleteEmailVerificationInput)
pub mod complete_email_verification_input {
	/// A builder for [`CompleteEmailVerificationInput`](crate::input::CompleteEmailVerificationInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) verification_id: std::option::Option<std::string::String>,
		pub(crate) code: std::option::Option<std::string::String>,
	}
	impl Builder {
		/// A universally unique identifier.
		pub fn verification_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.verification_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_verification_id(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.verification_id = input;
			self
		}
		/// The code sent to the requestee's email.
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		/// The code sent to the requestee's email.
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		/// Consumes the builder and constructs a [`CompleteEmailVerificationInput`](crate::input::CompleteEmailVerificationInput)
		pub fn build(
			self,
		) -> std::result::Result<
			crate::input::CompleteEmailVerificationInput,
			aws_smithy_http::operation::BuildError,
		> {
			Ok(crate::input::CompleteEmailVerificationInput {
				verification_id: self.verification_id,
				code: self.code,
			})
		}
	}
}
#[doc(hidden)]
pub type CompleteEmailVerificationInputOperationOutputAlias =
	crate::operation::CompleteEmailVerification;
#[doc(hidden)]
pub type CompleteEmailVerificationInputOperationRetryAlias = ();
impl CompleteEmailVerificationInput {
	/// Consumes the builder and constructs an Operation<[`CompleteEmailVerification`](crate::operation::CompleteEmailVerification)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::CompleteEmailVerification, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::CompleteEmailVerificationInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/identity/email/complete-verification")
					.expect("formatting should succeed");
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::CompleteEmailVerificationInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				Ok(builder.method("POST").uri(_uri))
			}
			let mut builder = update_http_builder(&self, _config, http::request::Builder::new())?;
			let mut builder = if let Some(auth) = &_config.auth {
				builder.header(http::header::AUTHORIZATION, auth.clone())
			} else {
				builder
			};
			builder = aws_smithy_http::header::set_request_header_if_absent(
				builder,
				http::header::CONTENT_TYPE,
				"application/json",
			);
			builder
		};
		let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
		#[allow(clippy::useless_conversion)]
		let body = aws_smithy_http::body::SdkBody::from(
			crate::operation_ser::serialize_operation_crate_operation_complete_email_verification(
				&self,
			)?,
		);
		if let Some(content_length) = body.content_length() {
			request = aws_smithy_http::header::set_request_header_if_absent(
				request,
				http::header::CONTENT_LENGTH,
				content_length,
			);
		}
		let request = request.body(body).expect("should be valid request");
		let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
		request
			.properties_mut()
			.insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
		let op = aws_smithy_http::operation::Operation::new(
			request,
			crate::operation::CompleteEmailVerification::new(),
		)
		.with_metadata(aws_smithy_http::operation::Metadata::new(
			"CompleteEmailVerification",
			"AuthService",
		));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`CompleteEmailVerificationInput`](crate::input::CompleteEmailVerificationInput)
	pub fn builder() -> crate::input::complete_email_verification_input::Builder {
		crate::input::complete_email_verification_input::Builder::default()
	}
}

/// See [`RefreshIdentityTokenInput`](crate::input::RefreshIdentityTokenInput)
pub mod refresh_identity_token_input {
	/// A builder for [`RefreshIdentityTokenInput`](crate::input::RefreshIdentityTokenInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) cookie: std::option::Option<std::vec::Vec<std::string::String>>,
		pub(crate) logout: std::option::Option<bool>,
	}
	impl Builder {
		/// Appends an item to `cookie`.
		///
		/// To override the contents of this collection use [`set_cookie`](Self::set_cookie).
		///
		/// Cookie values. Usually does not need to be manually set.
		pub fn cookie(mut self, input: impl Into<std::string::String>) -> Self {
			let mut v = self.cookie.unwrap_or_default();
			v.push(input.into());
			self.cookie = Some(v);
			self
		}
		/// Cookie values. Usually does not need to be manually set.
		pub fn set_cookie(
			mut self,
			input: std::option::Option<std::vec::Vec<std::string::String>>,
		) -> Self {
			self.cookie = input;
			self
		}
		/// When `true`, the current identity for the provided cookie will be logged out and a new identity will be returned.
		pub fn logout(mut self, input: bool) -> Self {
			self.logout = Some(input);
			self
		}
		/// When `true`, the current identity for the provided cookie will be logged out and a new identity will be returned.
		pub fn set_logout(mut self, input: std::option::Option<bool>) -> Self {
			self.logout = input;
			self
		}
		/// Consumes the builder and constructs a [`RefreshIdentityTokenInput`](crate::input::RefreshIdentityTokenInput)
		pub fn build(
			self,
		) -> std::result::Result<
			crate::input::RefreshIdentityTokenInput,
			aws_smithy_http::operation::BuildError,
		> {
			Ok(crate::input::RefreshIdentityTokenInput {
				cookie: self.cookie,
				logout: self.logout,
			})
		}
	}
}
#[doc(hidden)]
pub type RefreshIdentityTokenInputOperationOutputAlias = crate::operation::RefreshIdentityToken;
#[doc(hidden)]
pub type RefreshIdentityTokenInputOperationRetryAlias = ();
impl RefreshIdentityTokenInput {
	/// Consumes the builder and constructs an Operation<[`RefreshIdentityToken`](crate::operation::RefreshIdentityToken)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::RefreshIdentityToken, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::RefreshIdentityTokenInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/tokens/identity").expect("formatting should succeed");
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::RefreshIdentityTokenInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				let builder =
					crate::http_serde::add_headers_refresh_identity_token(input, builder)?;
				Ok(builder.method("POST").uri(_uri))
			}
			let mut builder = update_http_builder(&self, _config, http::request::Builder::new())?;
			let mut builder = if let Some(auth) = &_config.auth {
				builder.header(http::header::AUTHORIZATION, auth.clone())
			} else {
				builder
			};
			builder = aws_smithy_http::header::set_request_header_if_absent(
				builder,
				http::header::CONTENT_TYPE,
				"application/json",
			);
			builder
		};
		let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
		#[allow(clippy::useless_conversion)]
		let body = aws_smithy_http::body::SdkBody::from(
			crate::operation_ser::serialize_operation_crate_operation_refresh_identity_token(
				&self,
			)?,
		);
		if let Some(content_length) = body.content_length() {
			request = aws_smithy_http::header::set_request_header_if_absent(
				request,
				http::header::CONTENT_LENGTH,
				content_length,
			);
		}
		let request = request.body(body).expect("should be valid request");
		let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
		request
			.properties_mut()
			.insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
		let op = aws_smithy_http::operation::Operation::new(
			request,
			crate::operation::RefreshIdentityToken::new(),
		)
		.with_metadata(aws_smithy_http::operation::Metadata::new(
			"RefreshIdentityToken",
			"AuthService",
		));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`RefreshIdentityTokenInput`](crate::input::RefreshIdentityTokenInput)
	pub fn builder() -> crate::input::refresh_identity_token_input::Builder {
		crate::input::refresh_identity_token_input::Builder::default()
	}
}

/// See [`StartEmailVerificationInput`](crate::input::StartEmailVerificationInput)
pub mod start_email_verification_input {
	/// A builder for [`StartEmailVerificationInput`](crate::input::StartEmailVerificationInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) email: std::option::Option<std::string::String>,
		pub(crate) captcha: std::option::Option<crate::model::CaptchaConfig>,
		pub(crate) game_id: std::option::Option<std::string::String>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn email(mut self, input: impl Into<std::string::String>) -> Self {
			self.email = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_email(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.email = input;
			self
		}
		/// Methods to verify a captcha.
		pub fn captcha(mut self, input: crate::model::CaptchaConfig) -> Self {
			self.captcha = Some(input);
			self
		}
		/// Methods to verify a captcha.
		pub fn set_captcha(
			mut self,
			input: std::option::Option<crate::model::CaptchaConfig>,
		) -> Self {
			self.captcha = input;
			self
		}
		/// A universally unique identifier.
		pub fn game_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.game_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_game_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.game_id = input;
			self
		}
		/// Consumes the builder and constructs a [`StartEmailVerificationInput`](crate::input::StartEmailVerificationInput)
		pub fn build(
			self,
		) -> std::result::Result<
			crate::input::StartEmailVerificationInput,
			aws_smithy_http::operation::BuildError,
		> {
			Ok(crate::input::StartEmailVerificationInput {
				email: self.email,
				captcha: self.captcha,
				game_id: self.game_id,
			})
		}
	}
}
#[doc(hidden)]
pub type StartEmailVerificationInputOperationOutputAlias = crate::operation::StartEmailVerification;
#[doc(hidden)]
pub type StartEmailVerificationInputOperationRetryAlias = ();
impl StartEmailVerificationInput {
	/// Consumes the builder and constructs an Operation<[`StartEmailVerification`](crate::operation::StartEmailVerification)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::StartEmailVerification, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::StartEmailVerificationInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/identity/email/start-verification")
					.expect("formatting should succeed");
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::StartEmailVerificationInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				Ok(builder.method("POST").uri(_uri))
			}
			let mut builder = update_http_builder(&self, _config, http::request::Builder::new())?;
			let mut builder = if let Some(auth) = &_config.auth {
				builder.header(http::header::AUTHORIZATION, auth.clone())
			} else {
				builder
			};
			builder = aws_smithy_http::header::set_request_header_if_absent(
				builder,
				http::header::CONTENT_TYPE,
				"application/json",
			);
			builder
		};
		let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
		#[allow(clippy::useless_conversion)]
		let body = aws_smithy_http::body::SdkBody::from(
			crate::operation_ser::serialize_operation_crate_operation_start_email_verification(
				&self,
			)?,
		);
		if let Some(content_length) = body.content_length() {
			request = aws_smithy_http::header::set_request_header_if_absent(
				request,
				http::header::CONTENT_LENGTH,
				content_length,
			);
		}
		let request = request.body(body).expect("should be valid request");
		let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
		request
			.properties_mut()
			.insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
		let op = aws_smithy_http::operation::Operation::new(
			request,
			crate::operation::StartEmailVerification::new(),
		)
		.with_metadata(aws_smithy_http::operation::Metadata::new(
			"StartEmailVerification",
			"AuthService",
		));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`StartEmailVerificationInput`](crate::input::StartEmailVerificationInput)
	pub fn builder() -> crate::input::start_email_verification_input::Builder {
		crate::input::start_email_verification_input::Builder::default()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CompleteEmailVerificationInput {
	/// A universally unique identifier.
	pub verification_id: std::option::Option<std::string::String>,
	/// The code sent to the requestee's email.
	pub code: std::option::Option<std::string::String>,
}
impl CompleteEmailVerificationInput {
	/// A universally unique identifier.
	pub fn verification_id(&self) -> std::option::Option<&str> {
		self.verification_id.as_deref()
	}
	/// The code sent to the requestee's email.
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
}
impl std::fmt::Debug for CompleteEmailVerificationInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("CompleteEmailVerificationInput");
		formatter.field("verification_id", &self.verification_id);
		formatter.field("code", &self.code);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartEmailVerificationInput {
	#[allow(missing_docs)] // documentation missing in model
	pub email: std::option::Option<std::string::String>,
	/// Methods to verify a captcha.
	pub captcha: std::option::Option<crate::model::CaptchaConfig>,
	/// A universally unique identifier.
	pub game_id: std::option::Option<std::string::String>,
}
impl StartEmailVerificationInput {
	#[allow(missing_docs)] // documentation missing in model
	pub fn email(&self) -> std::option::Option<&str> {
		self.email.as_deref()
	}
	/// Methods to verify a captcha.
	pub fn captcha(&self) -> std::option::Option<&crate::model::CaptchaConfig> {
		self.captcha.as_ref()
	}
	/// A universally unique identifier.
	pub fn game_id(&self) -> std::option::Option<&str> {
		self.game_id.as_deref()
	}
}
impl std::fmt::Debug for StartEmailVerificationInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("StartEmailVerificationInput");
		formatter.field("email", &self.email);
		formatter.field("captcha", &self.captcha);
		formatter.field("game_id", &self.game_id);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RefreshIdentityTokenInput {
	/// Cookie values. Usually does not need to be manually set.
	pub cookie: std::option::Option<std::vec::Vec<std::string::String>>,
	/// When `true`, the current identity for the provided cookie will be logged out and a new identity will be returned.
	pub logout: std::option::Option<bool>,
}
impl RefreshIdentityTokenInput {
	/// Cookie values. Usually does not need to be manually set.
	pub fn cookie(&self) -> std::option::Option<&[std::string::String]> {
		self.cookie.as_deref()
	}
	/// When `true`, the current identity for the provided cookie will be logged out and a new identity will be returned.
	pub fn logout(&self) -> std::option::Option<bool> {
		self.logout
	}
}
impl std::fmt::Debug for RefreshIdentityTokenInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("RefreshIdentityTokenInput");
		formatter.field("cookie", &self.cookie);
		formatter.field("logout", &self.logout);
		formatter.finish()
	}
}
