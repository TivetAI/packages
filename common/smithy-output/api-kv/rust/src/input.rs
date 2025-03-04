// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`DeleteInput`](crate::input::DeleteInput)
pub mod delete_input {
	/// A builder for [`DeleteInput`](crate::input::DeleteInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) key: std::option::Option<std::string::String>,
		pub(crate) namespace_id: std::option::Option<std::string::String>,
	}
	impl Builder {
		/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
		pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
			self.key = Some(input.into());
			self
		}
		/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
		pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.key = input;
			self
		}
		/// A universally unique identifier.
		pub fn namespace_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.namespace_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_namespace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.namespace_id = input;
			self
		}
		/// Consumes the builder and constructs a [`DeleteInput`](crate::input::DeleteInput)
		pub fn build(
			self,
		) -> std::result::Result<crate::input::DeleteInput, aws_smithy_http::operation::BuildError>
		{
			Ok(crate::input::DeleteInput {
				key: self.key,
				namespace_id: self.namespace_id,
			})
		}
	}
}
#[doc(hidden)]
pub type DeleteInputOperationOutputAlias = crate::operation::Delete;
#[doc(hidden)]
pub type DeleteInputOperationRetryAlias = ();
impl DeleteInput {
	/// Consumes the builder and constructs an Operation<[`Delete`](crate::operation::Delete)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::Delete, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::DeleteInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/entries").expect("formatting should succeed");
				Ok(())
			}
			fn uri_query(
				_input: &crate::input::DeleteInput,
				mut output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				let mut query = aws_smithy_http::query::Writer::new(&mut output);
				if let Some(inner_1) = &_input.key {
					query.push_kv("key", &aws_smithy_http::query::fmt_string(inner_1));
				}
				if let Some(inner_2) = &_input.namespace_id {
					query.push_kv("namespace_id", &aws_smithy_http::query::fmt_string(inner_2));
				}
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::DeleteInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				uri_query(input, &mut _uri)?;
				Ok(builder.method("DELETE").uri(_uri))
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
		let op =
			aws_smithy_http::operation::Operation::new(request, crate::operation::Delete::new())
				.with_metadata(aws_smithy_http::operation::Metadata::new(
					"Delete",
					"KvService",
				));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`DeleteInput`](crate::input::DeleteInput)
	pub fn builder() -> crate::input::delete_input::Builder {
		crate::input::delete_input::Builder::default()
	}
}

/// See [`DeleteBatchInput`](crate::input::DeleteBatchInput)
pub mod delete_batch_input {
	/// A builder for [`DeleteBatchInput`](crate::input::DeleteBatchInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) keys: std::option::Option<std::vec::Vec<std::string::String>>,
		pub(crate) namespace_id: std::option::Option<std::string::String>,
	}
	impl Builder {
		/// Appends an item to `keys`.
		///
		/// To override the contents of this collection use [`set_keys`](Self::set_keys).
		///
		/// A list of keys.
		pub fn keys(mut self, input: impl Into<std::string::String>) -> Self {
			let mut v = self.keys.unwrap_or_default();
			v.push(input.into());
			self.keys = Some(v);
			self
		}
		/// A list of keys.
		pub fn set_keys(
			mut self,
			input: std::option::Option<std::vec::Vec<std::string::String>>,
		) -> Self {
			self.keys = input;
			self
		}
		/// A universally unique identifier.
		pub fn namespace_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.namespace_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_namespace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.namespace_id = input;
			self
		}
		/// Consumes the builder and constructs a [`DeleteBatchInput`](crate::input::DeleteBatchInput)
		pub fn build(
			self,
		) -> std::result::Result<
			crate::input::DeleteBatchInput,
			aws_smithy_http::operation::BuildError,
		> {
			Ok(crate::input::DeleteBatchInput {
				keys: self.keys,
				namespace_id: self.namespace_id,
			})
		}
	}
}
#[doc(hidden)]
pub type DeleteBatchInputOperationOutputAlias = crate::operation::DeleteBatch;
#[doc(hidden)]
pub type DeleteBatchInputOperationRetryAlias = ();
impl DeleteBatchInput {
	/// Consumes the builder and constructs an Operation<[`DeleteBatch`](crate::operation::DeleteBatch)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::DeleteBatch, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::DeleteBatchInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/entries/batch").expect("formatting should succeed");
				Ok(())
			}
			fn uri_query(
				_input: &crate::input::DeleteBatchInput,
				mut output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				let mut query = aws_smithy_http::query::Writer::new(&mut output);
				if let Some(inner_3) = &_input.keys {
					for inner_4 in inner_3 {
						query.push_kv("keys", &aws_smithy_http::query::fmt_string(inner_4));
					}
				}
				if let Some(inner_5) = &_input.namespace_id {
					query.push_kv("namespace_id", &aws_smithy_http::query::fmt_string(inner_5));
				}
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::DeleteBatchInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				uri_query(input, &mut _uri)?;
				Ok(builder.method("DELETE").uri(_uri))
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
			crate::operation::DeleteBatch::new(),
		)
		.with_metadata(aws_smithy_http::operation::Metadata::new(
			"DeleteBatch",
			"KvService",
		));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`DeleteBatchInput`](crate::input::DeleteBatchInput)
	pub fn builder() -> crate::input::delete_batch_input::Builder {
		crate::input::delete_batch_input::Builder::default()
	}
}

/// See [`GetInput`](crate::input::GetInput)
pub mod get_input {
	/// A builder for [`GetInput`](crate::input::GetInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) key: std::option::Option<std::string::String>,
		pub(crate) watch_index: std::option::Option<std::string::String>,
		pub(crate) namespace_id: std::option::Option<std::string::String>,
	}
	impl Builder {
		/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
		pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
			self.key = Some(input.into());
			self
		}
		/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
		pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.key = input;
			self
		}
		/// A query parameter denoting the requests watch index.
		pub fn watch_index(mut self, input: impl Into<std::string::String>) -> Self {
			self.watch_index = Some(input.into());
			self
		}
		/// A query parameter denoting the requests watch index.
		pub fn set_watch_index(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.watch_index = input;
			self
		}
		/// A universally unique identifier.
		pub fn namespace_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.namespace_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_namespace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.namespace_id = input;
			self
		}
		/// Consumes the builder and constructs a [`GetInput`](crate::input::GetInput)
		pub fn build(
			self,
		) -> std::result::Result<crate::input::GetInput, aws_smithy_http::operation::BuildError> {
			Ok(crate::input::GetInput {
				key: self.key,
				watch_index: self.watch_index,
				namespace_id: self.namespace_id,
			})
		}
	}
}
#[doc(hidden)]
pub type GetInputOperationOutputAlias = crate::operation::Get;
#[doc(hidden)]
pub type GetInputOperationRetryAlias = ();
impl GetInput {
	/// Consumes the builder and constructs an Operation<[`Get`](crate::operation::Get)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::Get, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::GetInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/entries").expect("formatting should succeed");
				Ok(())
			}
			fn uri_query(
				_input: &crate::input::GetInput,
				mut output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				let mut query = aws_smithy_http::query::Writer::new(&mut output);
				if let Some(inner_6) = &_input.key {
					query.push_kv("key", &aws_smithy_http::query::fmt_string(inner_6));
				}
				if let Some(inner_7) = &_input.watch_index {
					query.push_kv("watch_index", &aws_smithy_http::query::fmt_string(inner_7));
				}
				if let Some(inner_8) = &_input.namespace_id {
					query.push_kv("namespace_id", &aws_smithy_http::query::fmt_string(inner_8));
				}
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::GetInput,
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
		let op = aws_smithy_http::operation::Operation::new(request, crate::operation::Get::new())
			.with_metadata(aws_smithy_http::operation::Metadata::new(
				"Get",
				"KvService",
			));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`GetInput`](crate::input::GetInput)
	pub fn builder() -> crate::input::get_input::Builder {
		crate::input::get_input::Builder::default()
	}
}

/// See [`GetBatchInput`](crate::input::GetBatchInput)
pub mod get_batch_input {
	/// A builder for [`GetBatchInput`](crate::input::GetBatchInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) keys: std::option::Option<std::vec::Vec<std::string::String>>,
		pub(crate) watch_index: std::option::Option<std::string::String>,
		pub(crate) namespace_id: std::option::Option<std::string::String>,
	}
	impl Builder {
		/// Appends an item to `keys`.
		///
		/// To override the contents of this collection use [`set_keys`](Self::set_keys).
		///
		/// A list of keys.
		pub fn keys(mut self, input: impl Into<std::string::String>) -> Self {
			let mut v = self.keys.unwrap_or_default();
			v.push(input.into());
			self.keys = Some(v);
			self
		}
		/// A list of keys.
		pub fn set_keys(
			mut self,
			input: std::option::Option<std::vec::Vec<std::string::String>>,
		) -> Self {
			self.keys = input;
			self
		}
		/// A query parameter denoting the requests watch index.
		pub fn watch_index(mut self, input: impl Into<std::string::String>) -> Self {
			self.watch_index = Some(input.into());
			self
		}
		/// A query parameter denoting the requests watch index.
		pub fn set_watch_index(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.watch_index = input;
			self
		}
		/// A universally unique identifier.
		pub fn namespace_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.namespace_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_namespace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.namespace_id = input;
			self
		}
		/// Consumes the builder and constructs a [`GetBatchInput`](crate::input::GetBatchInput)
		pub fn build(
			self,
		) -> std::result::Result<crate::input::GetBatchInput, aws_smithy_http::operation::BuildError>
		{
			Ok(crate::input::GetBatchInput {
				keys: self.keys,
				watch_index: self.watch_index,
				namespace_id: self.namespace_id,
			})
		}
	}
}
#[doc(hidden)]
pub type GetBatchInputOperationOutputAlias = crate::operation::GetBatch;
#[doc(hidden)]
pub type GetBatchInputOperationRetryAlias = ();
impl GetBatchInput {
	/// Consumes the builder and constructs an Operation<[`GetBatch`](crate::operation::GetBatch)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::GetBatch, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::GetBatchInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/entries/batch").expect("formatting should succeed");
				Ok(())
			}
			fn uri_query(
				_input: &crate::input::GetBatchInput,
				mut output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				let mut query = aws_smithy_http::query::Writer::new(&mut output);
				if let Some(inner_9) = &_input.keys {
					for inner_10 in inner_9 {
						query.push_kv("keys", &aws_smithy_http::query::fmt_string(inner_10));
					}
				}
				if let Some(inner_11) = &_input.watch_index {
					query.push_kv("watch_index", &aws_smithy_http::query::fmt_string(inner_11));
				}
				if let Some(inner_12) = &_input.namespace_id {
					query.push_kv(
						"namespace_id",
						&aws_smithy_http::query::fmt_string(inner_12),
					);
				}
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::GetBatchInput,
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
		let op =
			aws_smithy_http::operation::Operation::new(request, crate::operation::GetBatch::new())
				.with_metadata(aws_smithy_http::operation::Metadata::new(
					"GetBatch",
					"KvService",
				));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`GetBatchInput`](crate::input::GetBatchInput)
	pub fn builder() -> crate::input::get_batch_input::Builder {
		crate::input::get_batch_input::Builder::default()
	}
}

/// See [`PutInput`](crate::input::PutInput)
pub mod put_input {
	/// A builder for [`PutInput`](crate::input::PutInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) namespace_id: std::option::Option<std::string::String>,
		pub(crate) key: std::option::Option<std::string::String>,
		pub(crate) value: std::option::Option<aws_smithy_types::Document>,
	}
	impl Builder {
		/// A universally unique identifier.
		pub fn namespace_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.namespace_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_namespace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.namespace_id = input;
			self
		}
		/// Any JSON value to set the key to.
		pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
			self.key = Some(input.into());
			self
		}
		/// Any JSON value to set the key to.
		pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.key = input;
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn value(mut self, input: aws_smithy_types::Document) -> Self {
			self.value = Some(input);
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_value(mut self, input: std::option::Option<aws_smithy_types::Document>) -> Self {
			self.value = input;
			self
		}
		/// Consumes the builder and constructs a [`PutInput`](crate::input::PutInput)
		pub fn build(
			self,
		) -> std::result::Result<crate::input::PutInput, aws_smithy_http::operation::BuildError> {
			Ok(crate::input::PutInput {
				namespace_id: self.namespace_id,
				key: self.key,
				value: self.value,
			})
		}
	}
}
#[doc(hidden)]
pub type PutInputOperationOutputAlias = crate::operation::Put;
#[doc(hidden)]
pub type PutInputOperationRetryAlias = ();
impl PutInput {
	/// Consumes the builder and constructs an Operation<[`Put`](crate::operation::Put)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::Put, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::PutInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/entries").expect("formatting should succeed");
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::PutInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				Ok(builder.method("PUT").uri(_uri))
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
			crate::operation_ser::serialize_operation_crate_operation_put(&self)?,
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
		let op = aws_smithy_http::operation::Operation::new(request, crate::operation::Put::new())
			.with_metadata(aws_smithy_http::operation::Metadata::new(
				"Put",
				"KvService",
			));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`PutInput`](crate::input::PutInput)
	pub fn builder() -> crate::input::put_input::Builder {
		crate::input::put_input::Builder::default()
	}
}

/// See [`PutBatchInput`](crate::input::PutBatchInput)
pub mod put_batch_input {
	/// A builder for [`PutBatchInput`](crate::input::PutBatchInput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) namespace_id: std::option::Option<std::string::String>,
		pub(crate) entries: std::option::Option<std::vec::Vec<crate::model::PutEntry>>,
	}
	impl Builder {
		/// A universally unique identifier.
		pub fn namespace_id(mut self, input: impl Into<std::string::String>) -> Self {
			self.namespace_id = Some(input.into());
			self
		}
		/// A universally unique identifier.
		pub fn set_namespace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.namespace_id = input;
			self
		}
		/// Appends an item to `entries`.
		///
		/// To override the contents of this collection use [`set_entries`](Self::set_entries).
		///
		/// A list of entries to insert.
		pub fn entries(mut self, input: crate::model::PutEntry) -> Self {
			let mut v = self.entries.unwrap_or_default();
			v.push(input);
			self.entries = Some(v);
			self
		}
		/// A list of entries to insert.
		pub fn set_entries(
			mut self,
			input: std::option::Option<std::vec::Vec<crate::model::PutEntry>>,
		) -> Self {
			self.entries = input;
			self
		}
		/// Consumes the builder and constructs a [`PutBatchInput`](crate::input::PutBatchInput)
		pub fn build(
			self,
		) -> std::result::Result<crate::input::PutBatchInput, aws_smithy_http::operation::BuildError>
		{
			Ok(crate::input::PutBatchInput {
				namespace_id: self.namespace_id,
				entries: self.entries,
			})
		}
	}
}
#[doc(hidden)]
pub type PutBatchInputOperationOutputAlias = crate::operation::PutBatch;
#[doc(hidden)]
pub type PutBatchInputOperationRetryAlias = ();
impl PutBatchInput {
	/// Consumes the builder and constructs an Operation<[`PutBatch`](crate::operation::PutBatch)>
	#[allow(unused_mut)]
	#[allow(clippy::let_and_return)]
	#[allow(clippy::needless_borrow)]
	pub async fn make_operation(
		&self,
		_config: &crate::config::Config,
	) -> std::result::Result<
		aws_smithy_http::operation::Operation<crate::operation::PutBatch, ()>,
		aws_smithy_http::operation::BuildError,
	> {
		let mut request = {
			fn uri_base(
				_input: &crate::input::PutBatchInput,
				output: &mut String,
			) -> Result<(), aws_smithy_http::operation::BuildError> {
				write!(output, "/entries/batch").expect("formatting should succeed");
				Ok(())
			}
			#[allow(clippy::unnecessary_wraps)]
			fn update_http_builder(
				input: &crate::input::PutBatchInput,
				_config: &crate::config::Config,
				builder: http::request::Builder,
			) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
			{
				let mut _uri = String::new();
				_uri = format!("{}{}", _config.uri.clone(), _uri);
				uri_base(input, &mut _uri)?;
				Ok(builder.method("PUT").uri(_uri))
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
			crate::operation_ser::serialize_operation_crate_operation_put_batch(&self)?,
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
		let op =
			aws_smithy_http::operation::Operation::new(request, crate::operation::PutBatch::new())
				.with_metadata(aws_smithy_http::operation::Metadata::new(
					"PutBatch",
					"KvService",
				));
		Ok(op)
	}
	/// Creates a new builder-style object to manufacture [`PutBatchInput`](crate::input::PutBatchInput)
	pub fn builder() -> crate::input::put_batch_input::Builder {
		crate::input::put_batch_input::Builder::default()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutBatchInput {
	/// A universally unique identifier.
	pub namespace_id: std::option::Option<std::string::String>,
	/// A list of entries to insert.
	pub entries: std::option::Option<std::vec::Vec<crate::model::PutEntry>>,
}
impl PutBatchInput {
	/// A universally unique identifier.
	pub fn namespace_id(&self) -> std::option::Option<&str> {
		self.namespace_id.as_deref()
	}
	/// A list of entries to insert.
	pub fn entries(&self) -> std::option::Option<&[crate::model::PutEntry]> {
		self.entries.as_deref()
	}
}
impl std::fmt::Debug for PutBatchInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("PutBatchInput");
		formatter.field("namespace_id", &self.namespace_id);
		formatter.field("entries", &self.entries);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteBatchInput {
	/// A list of keys.
	pub keys: std::option::Option<std::vec::Vec<std::string::String>>,
	/// A universally unique identifier.
	pub namespace_id: std::option::Option<std::string::String>,
}
impl DeleteBatchInput {
	/// A list of keys.
	pub fn keys(&self) -> std::option::Option<&[std::string::String]> {
		self.keys.as_deref()
	}
	/// A universally unique identifier.
	pub fn namespace_id(&self) -> std::option::Option<&str> {
		self.namespace_id.as_deref()
	}
}
impl std::fmt::Debug for DeleteBatchInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("DeleteBatchInput");
		formatter.field("keys", &self.keys);
		formatter.field("namespace_id", &self.namespace_id);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetBatchInput {
	/// A list of keys.
	pub keys: std::option::Option<std::vec::Vec<std::string::String>>,
	/// A query parameter denoting the requests watch index.
	pub watch_index: std::option::Option<std::string::String>,
	/// A universally unique identifier.
	pub namespace_id: std::option::Option<std::string::String>,
}
impl GetBatchInput {
	/// A list of keys.
	pub fn keys(&self) -> std::option::Option<&[std::string::String]> {
		self.keys.as_deref()
	}
	/// A query parameter denoting the requests watch index.
	pub fn watch_index(&self) -> std::option::Option<&str> {
		self.watch_index.as_deref()
	}
	/// A universally unique identifier.
	pub fn namespace_id(&self) -> std::option::Option<&str> {
		self.namespace_id.as_deref()
	}
}
impl std::fmt::Debug for GetBatchInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("GetBatchInput");
		formatter.field("keys", &self.keys);
		formatter.field("watch_index", &self.watch_index);
		formatter.field("namespace_id", &self.namespace_id);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutInput {
	/// A universally unique identifier.
	pub namespace_id: std::option::Option<std::string::String>,
	/// Any JSON value to set the key to.
	pub key: std::option::Option<std::string::String>,
	#[allow(missing_docs)] // documentation missing in model
	pub value: std::option::Option<aws_smithy_types::Document>,
}
impl PutInput {
	/// A universally unique identifier.
	pub fn namespace_id(&self) -> std::option::Option<&str> {
		self.namespace_id.as_deref()
	}
	/// Any JSON value to set the key to.
	pub fn key(&self) -> std::option::Option<&str> {
		self.key.as_deref()
	}
	#[allow(missing_docs)] // documentation missing in model
	pub fn value(&self) -> std::option::Option<&aws_smithy_types::Document> {
		self.value.as_ref()
	}
}
impl std::fmt::Debug for PutInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("PutInput");
		formatter.field("namespace_id", &self.namespace_id);
		formatter.field("key", &self.key);
		formatter.field("value", &self.value);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteInput {
	/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
	pub key: std::option::Option<std::string::String>,
	/// A universally unique identifier.
	pub namespace_id: std::option::Option<std::string::String>,
}
impl DeleteInput {
	/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
	pub fn key(&self) -> std::option::Option<&str> {
		self.key.as_deref()
	}
	/// A universally unique identifier.
	pub fn namespace_id(&self) -> std::option::Option<&str> {
		self.namespace_id.as_deref()
	}
}
impl std::fmt::Debug for DeleteInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("DeleteInput");
		formatter.field("key", &self.key);
		formatter.field("namespace_id", &self.namespace_id);
		formatter.finish()
	}
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetInput {
	/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
	pub key: std::option::Option<std::string::String>,
	/// A query parameter denoting the requests watch index.
	pub watch_index: std::option::Option<std::string::String>,
	/// A universally unique identifier.
	pub namespace_id: std::option::Option<std::string::String>,
}
impl GetInput {
	/// A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `["a", "b", "c"]`). Slashes can be escaped by using a forward slash (e.g. `a/b\/c/d` has the path components `["a", "b/c", "d"]`). See `tivet.api.kv.common#KeyComponents` for the structure of a `tivet.api.kv.common#Key` split by `/`.
	pub fn key(&self) -> std::option::Option<&str> {
		self.key.as_deref()
	}
	/// A query parameter denoting the requests watch index.
	pub fn watch_index(&self) -> std::option::Option<&str> {
		self.watch_index.as_deref()
	}
	/// A universally unique identifier.
	pub fn namespace_id(&self) -> std::option::Option<&str> {
		self.namespace_id.as_deref()
	}
}
impl std::fmt::Debug for GetInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("GetInput");
		formatter.field("key", &self.key);
		formatter.field("watch_index", &self.watch_index);
		formatter.field("namespace_id", &self.namespace_id);
		formatter.finish()
	}
}
