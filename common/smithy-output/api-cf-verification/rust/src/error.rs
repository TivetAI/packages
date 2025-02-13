// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `VerifyCustomHostname` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct VerifyCustomHostnameError {
	/// Kind of error that occurred.
	pub kind: VerifyCustomHostnameErrorKind,
	/// Additional metadata about the error, including error code, message, and request ID.
	pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `VerifyCustomHostname` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum VerifyCustomHostnameErrorKind {
	/// An error caused by internal server problems.
	InternalError(crate::error::InternalError),
	/// An error thrown when the requestee has hit a rate limit. You are sending too many requests too quickly.
	RateLimitError(crate::error::RateLimitError),
	/// An error thrown when the requestee requests a resource they do not have access to.
	ForbiddenError(crate::error::ForbiddenError),
	/// An error thrown when the requestee is not authenticated.
	UnauthorizedError(crate::error::UnauthorizedError),
	/// An error thrown when the requestee requests a non existent resource.
	NotFoundError(crate::error::NotFoundError),
	/// An error thrown when the requestee has sent an invalid or malformed request.
	BadRequestError(crate::error::BadRequestError),
	/// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
	Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for VerifyCustomHostnameError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.kind {
			VerifyCustomHostnameErrorKind::InternalError(_inner) => _inner.fmt(f),
			VerifyCustomHostnameErrorKind::RateLimitError(_inner) => _inner.fmt(f),
			VerifyCustomHostnameErrorKind::ForbiddenError(_inner) => _inner.fmt(f),
			VerifyCustomHostnameErrorKind::UnauthorizedError(_inner) => _inner.fmt(f),
			VerifyCustomHostnameErrorKind::NotFoundError(_inner) => _inner.fmt(f),
			VerifyCustomHostnameErrorKind::BadRequestError(_inner) => _inner.fmt(f),
			VerifyCustomHostnameErrorKind::Unhandled(_inner) => _inner.fmt(f),
		}
	}
}
impl aws_smithy_types::retry::ProvideErrorKind for VerifyCustomHostnameError {
	fn code(&self) -> Option<&str> {
		VerifyCustomHostnameError::code(self)
	}
	fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
		match &self.kind {
			VerifyCustomHostnameErrorKind::InternalError(inner) => {
				Some(inner.retryable_error_kind())
			}
			VerifyCustomHostnameErrorKind::UnauthorizedError(inner) => {
				Some(inner.retryable_error_kind())
			}
			_ => None,
		}
	}
}
impl VerifyCustomHostnameError {
	/// Creates a new `VerifyCustomHostnameError`.
	pub fn new(kind: VerifyCustomHostnameErrorKind, meta: aws_smithy_types::Error) -> Self {
		Self { kind, meta }
	}

	/// Creates the `VerifyCustomHostnameError::Unhandled` variant from any error type.
	pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
		Self {
			kind: VerifyCustomHostnameErrorKind::Unhandled(err.into()),
			meta: Default::default(),
		}
	}

	/// Creates the `VerifyCustomHostnameError::Unhandled` variant from a `aws_smithy_types::Error`.
	pub fn generic(err: aws_smithy_types::Error) -> Self {
		Self {
			meta: err.clone(),
			kind: VerifyCustomHostnameErrorKind::Unhandled(err.into()),
		}
	}

	/// Returns the error message if one is available.
	pub fn message(&self) -> Option<&str> {
		self.meta.message()
	}

	/// Returns error metadata, which includes the error code, message,
	/// request ID, and potentially additional information.
	pub fn meta(&self) -> &aws_smithy_types::Error {
		&self.meta
	}

	/// Returns the request ID if it's available.
	pub fn request_id(&self) -> Option<&str> {
		self.meta.request_id()
	}

	/// Returns the error code if it's available.
	pub fn code(&self) -> Option<&str> {
		self.meta.code()
	}
	/// Returns `true` if the error kind is `VerifyCustomHostnameErrorKind::InternalError`.
	pub fn is_internal_error(&self) -> bool {
		matches!(&self.kind, VerifyCustomHostnameErrorKind::InternalError(_))
	}
	/// Returns `true` if the error kind is `VerifyCustomHostnameErrorKind::RateLimitError`.
	pub fn is_rate_limit_error(&self) -> bool {
		matches!(&self.kind, VerifyCustomHostnameErrorKind::RateLimitError(_))
	}
	/// Returns `true` if the error kind is `VerifyCustomHostnameErrorKind::ForbiddenError`.
	pub fn is_forbidden_error(&self) -> bool {
		matches!(&self.kind, VerifyCustomHostnameErrorKind::ForbiddenError(_))
	}
	/// Returns `true` if the error kind is `VerifyCustomHostnameErrorKind::UnauthorizedError`.
	pub fn is_unauthorized_error(&self) -> bool {
		matches!(
			&self.kind,
			VerifyCustomHostnameErrorKind::UnauthorizedError(_)
		)
	}
	/// Returns `true` if the error kind is `VerifyCustomHostnameErrorKind::NotFoundError`.
	pub fn is_not_found_error(&self) -> bool {
		matches!(&self.kind, VerifyCustomHostnameErrorKind::NotFoundError(_))
	}
	/// Returns `true` if the error kind is `VerifyCustomHostnameErrorKind::BadRequestError`.
	pub fn is_bad_request_error(&self) -> bool {
		matches!(
			&self.kind,
			VerifyCustomHostnameErrorKind::BadRequestError(_)
		)
	}
}
impl std::error::Error for VerifyCustomHostnameError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match &self.kind {
			VerifyCustomHostnameErrorKind::InternalError(_inner) => Some(_inner),
			VerifyCustomHostnameErrorKind::RateLimitError(_inner) => Some(_inner),
			VerifyCustomHostnameErrorKind::ForbiddenError(_inner) => Some(_inner),
			VerifyCustomHostnameErrorKind::UnauthorizedError(_inner) => Some(_inner),
			VerifyCustomHostnameErrorKind::NotFoundError(_inner) => Some(_inner),
			VerifyCustomHostnameErrorKind::BadRequestError(_inner) => Some(_inner),
			VerifyCustomHostnameErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
		}
	}
}

/// An error thrown when the requestee has sent an invalid or malformed request.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BadRequestError {
	#[allow(missing_docs)] // documentation missing in model
	pub code: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub message: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub documentation: std::option::Option<std::string::String>,
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub metadata: std::option::Option<aws_smithy_types::Document>,
}
impl BadRequestError {
	#[allow(missing_docs)] // documentation missing in model
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn documentation(&self) -> std::option::Option<&str> {
		self.documentation.as_deref()
	}
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub fn metadata(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.metadata.as_ref()
	}
}
impl std::fmt::Debug for BadRequestError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("BadRequestError");
		formatter.field("code", &self.code);
		formatter.field("message", &self.message);
		formatter.field("documentation", &self.documentation);
		formatter.field("metadata", &self.metadata);
		formatter.finish()
	}
}
impl BadRequestError {
	/// Returns the error message.
	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}
}
impl std::fmt::Display for BadRequestError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "BadRequestError")?;
		if let Some(inner_1) = &self.message {
			write!(f, ": {}", inner_1)?;
		}
		Ok(())
	}
}
impl std::error::Error for BadRequestError {}
/// See [`BadRequestError`](crate::error::BadRequestError)
pub mod bad_request_error {
	/// A builder for [`BadRequestError`](crate::error::BadRequestError)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) code: std::option::Option<std::string::String>,
		pub(crate) message: std::option::Option<std::string::String>,
		pub(crate) documentation: std::option::Option<std::string::String>,
		pub(crate) metadata: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
			self.message = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.message = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn documentation(mut self, input: impl Into<std::string::String>) -> Self {
			self.documentation = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_documentation(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.documentation = input;
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn metadata(mut self, input: aws_smithy_types::Document) -> Self {
			self.metadata = Some(input);
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn set_metadata(
			mut self,
			input: std::option::Option<aws_smithy_types::Document>,
		) -> Self {
			self.metadata = input;
			self
		}
		/// Consumes the builder and constructs a [`BadRequestError`](crate::error::BadRequestError)
		pub fn build(self) -> crate::error::BadRequestError {
			crate::error::BadRequestError {
				code: self.code,
				message: self.message,
				documentation: self.documentation,
				metadata: self.metadata,
			}
		}
	}
}
impl BadRequestError {
	/// Creates a new builder-style object to manufacture [`BadRequestError`](crate::error::BadRequestError)
	pub fn builder() -> crate::error::bad_request_error::Builder {
		crate::error::bad_request_error::Builder::default()
	}
}

/// An error thrown when the requestee requests a non existent resource.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NotFoundError {
	#[allow(missing_docs)] // documentation missing in model
	pub code: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub message: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub documentation: std::option::Option<std::string::String>,
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub metadata: std::option::Option<aws_smithy_types::Document>,
}
impl NotFoundError {
	#[allow(missing_docs)] // documentation missing in model
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn documentation(&self) -> std::option::Option<&str> {
		self.documentation.as_deref()
	}
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub fn metadata(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.metadata.as_ref()
	}
}
impl std::fmt::Debug for NotFoundError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("NotFoundError");
		formatter.field("code", &self.code);
		formatter.field("message", &self.message);
		formatter.field("documentation", &self.documentation);
		formatter.field("metadata", &self.metadata);
		formatter.finish()
	}
}
impl NotFoundError {
	/// Returns the error message.
	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}
}
impl std::fmt::Display for NotFoundError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "NotFoundError")?;
		if let Some(inner_2) = &self.message {
			write!(f, ": {}", inner_2)?;
		}
		Ok(())
	}
}
impl std::error::Error for NotFoundError {}
/// See [`NotFoundError`](crate::error::NotFoundError)
pub mod not_found_error {
	/// A builder for [`NotFoundError`](crate::error::NotFoundError)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) code: std::option::Option<std::string::String>,
		pub(crate) message: std::option::Option<std::string::String>,
		pub(crate) documentation: std::option::Option<std::string::String>,
		pub(crate) metadata: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
			self.message = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.message = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn documentation(mut self, input: impl Into<std::string::String>) -> Self {
			self.documentation = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_documentation(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.documentation = input;
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn metadata(mut self, input: aws_smithy_types::Document) -> Self {
			self.metadata = Some(input);
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn set_metadata(
			mut self,
			input: std::option::Option<aws_smithy_types::Document>,
		) -> Self {
			self.metadata = input;
			self
		}
		/// Consumes the builder and constructs a [`NotFoundError`](crate::error::NotFoundError)
		pub fn build(self) -> crate::error::NotFoundError {
			crate::error::NotFoundError {
				code: self.code,
				message: self.message,
				documentation: self.documentation,
				metadata: self.metadata,
			}
		}
	}
}
impl NotFoundError {
	/// Creates a new builder-style object to manufacture [`NotFoundError`](crate::error::NotFoundError)
	pub fn builder() -> crate::error::not_found_error::Builder {
		crate::error::not_found_error::Builder::default()
	}
}

/// An error thrown when the requestee is not authenticated.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UnauthorizedError {
	#[allow(missing_docs)] // documentation missing in model
	pub code: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub message: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub documentation: std::option::Option<std::string::String>,
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub metadata: std::option::Option<aws_smithy_types::Document>,
}
impl UnauthorizedError {
	#[allow(missing_docs)] // documentation missing in model
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn documentation(&self) -> std::option::Option<&str> {
		self.documentation.as_deref()
	}
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub fn metadata(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.metadata.as_ref()
	}
}
impl std::fmt::Debug for UnauthorizedError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("UnauthorizedError");
		formatter.field("code", &self.code);
		formatter.field("message", &self.message);
		formatter.field("documentation", &self.documentation);
		formatter.field("metadata", &self.metadata);
		formatter.finish()
	}
}
impl UnauthorizedError {
	/// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
	pub fn retryable_error_kind(&self) -> aws_smithy_types::retry::ErrorKind {
		aws_smithy_types::retry::ErrorKind::ClientError
	}
	/// Returns the error message.
	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}
}
impl std::fmt::Display for UnauthorizedError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "UnauthorizedError")?;
		if let Some(inner_3) = &self.message {
			write!(f, ": {}", inner_3)?;
		}
		Ok(())
	}
}
impl std::error::Error for UnauthorizedError {}
/// See [`UnauthorizedError`](crate::error::UnauthorizedError)
pub mod unauthorized_error {
	/// A builder for [`UnauthorizedError`](crate::error::UnauthorizedError)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) code: std::option::Option<std::string::String>,
		pub(crate) message: std::option::Option<std::string::String>,
		pub(crate) documentation: std::option::Option<std::string::String>,
		pub(crate) metadata: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
			self.message = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.message = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn documentation(mut self, input: impl Into<std::string::String>) -> Self {
			self.documentation = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_documentation(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.documentation = input;
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn metadata(mut self, input: aws_smithy_types::Document) -> Self {
			self.metadata = Some(input);
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn set_metadata(
			mut self,
			input: std::option::Option<aws_smithy_types::Document>,
		) -> Self {
			self.metadata = input;
			self
		}
		/// Consumes the builder and constructs a [`UnauthorizedError`](crate::error::UnauthorizedError)
		pub fn build(self) -> crate::error::UnauthorizedError {
			crate::error::UnauthorizedError {
				code: self.code,
				message: self.message,
				documentation: self.documentation,
				metadata: self.metadata,
			}
		}
	}
}
impl UnauthorizedError {
	/// Creates a new builder-style object to manufacture [`UnauthorizedError`](crate::error::UnauthorizedError)
	pub fn builder() -> crate::error::unauthorized_error::Builder {
		crate::error::unauthorized_error::Builder::default()
	}
}

/// An error thrown when the requestee requests a resource they do not have access to.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ForbiddenError {
	#[allow(missing_docs)] // documentation missing in model
	pub code: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub message: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub documentation: std::option::Option<std::string::String>,
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub metadata: std::option::Option<aws_smithy_types::Document>,
}
impl ForbiddenError {
	#[allow(missing_docs)] // documentation missing in model
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn documentation(&self) -> std::option::Option<&str> {
		self.documentation.as_deref()
	}
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub fn metadata(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.metadata.as_ref()
	}
}
impl std::fmt::Debug for ForbiddenError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("ForbiddenError");
		formatter.field("code", &self.code);
		formatter.field("message", &self.message);
		formatter.field("documentation", &self.documentation);
		formatter.field("metadata", &self.metadata);
		formatter.finish()
	}
}
impl ForbiddenError {
	/// Returns the error message.
	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}
}
impl std::fmt::Display for ForbiddenError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "ForbiddenError")?;
		if let Some(inner_4) = &self.message {
			write!(f, ": {}", inner_4)?;
		}
		Ok(())
	}
}
impl std::error::Error for ForbiddenError {}
/// See [`ForbiddenError`](crate::error::ForbiddenError)
pub mod forbidden_error {
	/// A builder for [`ForbiddenError`](crate::error::ForbiddenError)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) code: std::option::Option<std::string::String>,
		pub(crate) message: std::option::Option<std::string::String>,
		pub(crate) documentation: std::option::Option<std::string::String>,
		pub(crate) metadata: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
			self.message = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.message = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn documentation(mut self, input: impl Into<std::string::String>) -> Self {
			self.documentation = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_documentation(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.documentation = input;
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn metadata(mut self, input: aws_smithy_types::Document) -> Self {
			self.metadata = Some(input);
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn set_metadata(
			mut self,
			input: std::option::Option<aws_smithy_types::Document>,
		) -> Self {
			self.metadata = input;
			self
		}
		/// Consumes the builder and constructs a [`ForbiddenError`](crate::error::ForbiddenError)
		pub fn build(self) -> crate::error::ForbiddenError {
			crate::error::ForbiddenError {
				code: self.code,
				message: self.message,
				documentation: self.documentation,
				metadata: self.metadata,
			}
		}
	}
}
impl ForbiddenError {
	/// Creates a new builder-style object to manufacture [`ForbiddenError`](crate::error::ForbiddenError)
	pub fn builder() -> crate::error::forbidden_error::Builder {
		crate::error::forbidden_error::Builder::default()
	}
}

/// An error thrown when the requestee has hit a rate limit. You are sending too many requests too quickly.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RateLimitError {
	#[allow(missing_docs)] // documentation missing in model
	pub code: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub message: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub documentation: std::option::Option<std::string::String>,
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub metadata: std::option::Option<aws_smithy_types::Document>,
}
impl RateLimitError {
	#[allow(missing_docs)] // documentation missing in model
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn documentation(&self) -> std::option::Option<&str> {
		self.documentation.as_deref()
	}
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub fn metadata(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.metadata.as_ref()
	}
}
impl std::fmt::Debug for RateLimitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("RateLimitError");
		formatter.field("code", &self.code);
		formatter.field("message", &self.message);
		formatter.field("documentation", &self.documentation);
		formatter.field("metadata", &self.metadata);
		formatter.finish()
	}
}
impl RateLimitError {
	/// Returns the error message.
	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}
}
impl std::fmt::Display for RateLimitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "RateLimitError")?;
		if let Some(inner_5) = &self.message {
			write!(f, ": {}", inner_5)?;
		}
		Ok(())
	}
}
impl std::error::Error for RateLimitError {}
/// See [`RateLimitError`](crate::error::RateLimitError)
pub mod rate_limit_error {
	/// A builder for [`RateLimitError`](crate::error::RateLimitError)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) code: std::option::Option<std::string::String>,
		pub(crate) message: std::option::Option<std::string::String>,
		pub(crate) documentation: std::option::Option<std::string::String>,
		pub(crate) metadata: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
			self.message = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.message = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn documentation(mut self, input: impl Into<std::string::String>) -> Self {
			self.documentation = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_documentation(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.documentation = input;
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn metadata(mut self, input: aws_smithy_types::Document) -> Self {
			self.metadata = Some(input);
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn set_metadata(
			mut self,
			input: std::option::Option<aws_smithy_types::Document>,
		) -> Self {
			self.metadata = input;
			self
		}
		/// Consumes the builder and constructs a [`RateLimitError`](crate::error::RateLimitError)
		pub fn build(self) -> crate::error::RateLimitError {
			crate::error::RateLimitError {
				code: self.code,
				message: self.message,
				documentation: self.documentation,
				metadata: self.metadata,
			}
		}
	}
}
impl RateLimitError {
	/// Creates a new builder-style object to manufacture [`RateLimitError`](crate::error::RateLimitError)
	pub fn builder() -> crate::error::rate_limit_error::Builder {
		crate::error::rate_limit_error::Builder::default()
	}
}

/// An error caused by internal server problems.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalError {
	#[allow(missing_docs)] // documentation missing in model
	pub code: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub message: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub documentation: std::option::Option<std::string::String>,
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub metadata: std::option::Option<aws_smithy_types::Document>,
}
impl InternalError {
	#[allow(missing_docs)] // documentation missing in model
	pub fn code(&self) -> std::option::Option<&str> {
		self.code.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn documentation(&self) -> std::option::Option<&str> {
		self.documentation.as_deref()
	}
	/// Unstructured metadata relating to an error. Must be manually parsed.
	pub fn metadata(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.metadata.as_ref()
	}
}
impl std::fmt::Debug for InternalError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("InternalError");
		formatter.field("code", &self.code);
		formatter.field("message", &self.message);
		formatter.field("documentation", &self.documentation);
		formatter.field("metadata", &self.metadata);
		formatter.finish()
	}
}
impl InternalError {
	/// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
	pub fn retryable_error_kind(&self) -> aws_smithy_types::retry::ErrorKind {
		aws_smithy_types::retry::ErrorKind::ServerError
	}
	/// Returns the error message.
	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}
}
impl std::fmt::Display for InternalError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "InternalError")?;
		if let Some(inner_6) = &self.message {
			write!(f, ": {}", inner_6)?;
		}
		Ok(())
	}
}
impl std::error::Error for InternalError {}
/// See [`InternalError`](crate::error::InternalError)
pub mod internal_error {
	/// A builder for [`InternalError`](crate::error::InternalError)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) code: std::option::Option<std::string::String>,
		pub(crate) message: std::option::Option<std::string::String>,
		pub(crate) documentation: std::option::Option<std::string::String>,
		pub(crate) metadata: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
			self.code = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.code = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
			self.message = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.message = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn documentation(mut self, input: impl Into<std::string::String>) -> Self {
			self.documentation = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_documentation(
			mut self,
			input: std::option::Option<std::string::String>,
		) -> Self {
			self.documentation = input;
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn metadata(mut self, input: aws_smithy_types::Document) -> Self {
			self.metadata = Some(input);
			self
		}
		/// Unstructured metadata relating to an error. Must be manually parsed.
		pub fn set_metadata(
			mut self,
			input: std::option::Option<aws_smithy_types::Document>,
		) -> Self {
			self.metadata = input;
			self
		}
		/// Consumes the builder and constructs a [`InternalError`](crate::error::InternalError)
		pub fn build(self) -> crate::error::InternalError {
			crate::error::InternalError {
				code: self.code,
				message: self.message,
				documentation: self.documentation,
				metadata: self.metadata,
			}
		}
	}
}
impl InternalError {
	/// Creates a new builder-style object to manufacture [`InternalError`](crate::error::InternalError)
	pub fn builder() -> crate::error::internal_error::Builder {
		crate::error::internal_error::Builder::default()
	}
}
