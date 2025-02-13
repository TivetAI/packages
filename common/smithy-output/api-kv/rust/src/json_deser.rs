// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
	response: &http::Response<bytes::Bytes>,
) -> Result<aws_smithy_types::Error, aws_smithy_json::deserialize::Error> {
	crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_crate_error_internal_error_json_err(
	value: &[u8],
	mut builder: crate::error::internal_error::Builder,
) -> Result<crate::error::internal_error::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"code" => {
						builder = builder.set_code(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"message" => {
						builder = builder.set_message(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"documentation" => {
						builder = builder.set_documentation(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"metadata" => {
						builder = builder.set_metadata(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_structure_crate_error_rate_limit_error_json_err(
	value: &[u8],
	mut builder: crate::error::rate_limit_error::Builder,
) -> Result<crate::error::rate_limit_error::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"code" => {
						builder = builder.set_code(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"message" => {
						builder = builder.set_message(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"documentation" => {
						builder = builder.set_documentation(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"metadata" => {
						builder = builder.set_metadata(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_structure_crate_error_forbidden_error_json_err(
	value: &[u8],
	mut builder: crate::error::forbidden_error::Builder,
) -> Result<crate::error::forbidden_error::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"code" => {
						builder = builder.set_code(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"message" => {
						builder = builder.set_message(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"documentation" => {
						builder = builder.set_documentation(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"metadata" => {
						builder = builder.set_metadata(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_structure_crate_error_unauthorized_error_json_err(
	value: &[u8],
	mut builder: crate::error::unauthorized_error::Builder,
) -> Result<crate::error::unauthorized_error::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"code" => {
						builder = builder.set_code(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"message" => {
						builder = builder.set_message(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"documentation" => {
						builder = builder.set_documentation(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"metadata" => {
						builder = builder.set_metadata(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_structure_crate_error_not_found_error_json_err(
	value: &[u8],
	mut builder: crate::error::not_found_error::Builder,
) -> Result<crate::error::not_found_error::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"code" => {
						builder = builder.set_code(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"message" => {
						builder = builder.set_message(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"documentation" => {
						builder = builder.set_documentation(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"metadata" => {
						builder = builder.set_metadata(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_structure_crate_error_bad_request_error_json_err(
	value: &[u8],
	mut builder: crate::error::bad_request_error::Builder,
) -> Result<crate::error::bad_request_error::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"code" => {
						builder = builder.set_code(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"message" => {
						builder = builder.set_message(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"documentation" => {
						builder = builder.set_documentation(
							aws_smithy_json::deserialize::token::expect_string_or_null(
								tokens.next(),
							)?
							.map(|s| s.to_unescaped().map(|u| u.into_owned()))
							.transpose()?,
						);
					}
					"metadata" => {
						builder = builder.set_metadata(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_operation_crate_operation_get(
	value: &[u8],
	mut builder: crate::output::get_output::Builder,
) -> Result<crate::output::get_output::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"deleted" => {
						builder = builder.set_deleted(
							aws_smithy_json::deserialize::token::expect_bool_or_null(
								tokens.next(),
							)?,
						);
					}
					"value" => {
						builder = builder.set_value(Some(
							aws_smithy_json::deserialize::token::expect_document(tokens)?,
						));
					}
					"watch" => {
						builder = builder.set_watch(
							crate::json_deser::deser_structure_crate_model_watch_response(tokens)?,
						);
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn deser_operation_crate_operation_get_batch(
	value: &[u8],
	mut builder: crate::output::get_batch_output::Builder,
) -> Result<crate::output::get_batch_output::Builder, aws_smithy_json::deserialize::Error> {
	let mut tokens_owned =
		aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
			.peekable();
	let tokens = &mut tokens_owned;
	aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
	loop {
		match tokens.next().transpose()? {
			Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
			Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
				match key.to_unescaped()?.as_ref() {
					"entries" => {
						builder = builder.set_entries(
							crate::json_deser::deser_list_tivet_api_kv_common_kv_entries(tokens)?,
						);
					}
					"watch" => {
						builder = builder.set_watch(
							crate::json_deser::deser_structure_crate_model_watch_response(tokens)?,
						);
					}
					_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
				}
			}
			other => {
				return Err(aws_smithy_json::deserialize::Error::custom(format!(
					"expected object key or end object, found: {:?}",
					other
				)))
			}
		}
	}
	if tokens.next().is_some() {
		return Err(aws_smithy_json::deserialize::Error::custom(
			"found more JSON tokens after completing parsing",
		));
	}
	Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
	if data.is_empty() {
		b"{}"
	} else {
		data
	}
}

pub fn deser_structure_crate_model_watch_response<'a, I>(
	tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::WatchResponse>, aws_smithy_json::deserialize::Error>
where
	I: Iterator<
		Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
	>,
{
	match tokens.next().transpose()? {
		Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
		Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
			#[allow(unused_mut)]
			let mut builder = crate::model::WatchResponse::builder();
			loop {
				match tokens.next().transpose()? {
					Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
					Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
						match key.to_unescaped()?.as_ref() {
							"index" => {
								builder = builder.set_index(
									aws_smithy_json::deserialize::token::expect_string_or_null(
										tokens.next(),
									)?
									.map(|s| s.to_unescaped().map(|u| u.into_owned()))
									.transpose()?,
								);
							}
							_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
						}
					}
					other => {
						return Err(aws_smithy_json::deserialize::Error::custom(format!(
							"expected object key or end object, found: {:?}",
							other
						)))
					}
				}
			}
			Ok(Some(builder.build()))
		}
		_ => Err(aws_smithy_json::deserialize::Error::custom(
			"expected start object or null",
		)),
	}
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_tivet_api_kv_common_kv_entries<'a, I>(
	tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::KvEntry>>, aws_smithy_json::deserialize::Error>
where
	I: Iterator<
		Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
	>,
{
	match tokens.next().transpose()? {
		Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
		Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
			let mut items = Vec::new();
			loop {
				match tokens.peek() {
					Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
						tokens.next().transpose().unwrap();
						break;
					}
					_ => {
						let value =
							crate::json_deser::deser_structure_crate_model_kv_entry(tokens)?;
						if let Some(value) = value {
							items.push(value);
						}
					}
				}
			}
			Ok(Some(items))
		}
		_ => Err(aws_smithy_json::deserialize::Error::custom(
			"expected start array or null",
		)),
	}
}

pub fn deser_structure_crate_model_kv_entry<'a, I>(
	tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::KvEntry>, aws_smithy_json::deserialize::Error>
where
	I: Iterator<
		Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
	>,
{
	match tokens.next().transpose()? {
		Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
		Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
			#[allow(unused_mut)]
			let mut builder = crate::model::KvEntry::builder();
			loop {
				match tokens.next().transpose()? {
					Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
					Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
						match key.to_unescaped()?.as_ref() {
							"key" => {
								builder = builder.set_key(
                                    crate::json_deser::deser_list_tivet_api_kv_common_key_components(tokens)?
                                );
							}
							"value" => {
								builder = builder.set_value(Some(
									aws_smithy_json::deserialize::token::expect_document(tokens)?,
								));
							}
							"deleted" => {
								builder = builder.set_deleted(
									aws_smithy_json::deserialize::token::expect_bool_or_null(
										tokens.next(),
									)?,
								);
							}
							_ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
						}
					}
					other => {
						return Err(aws_smithy_json::deserialize::Error::custom(format!(
							"expected object key or end object, found: {:?}",
							other
						)))
					}
				}
			}
			Ok(Some(builder.build()))
		}
		_ => Err(aws_smithy_json::deserialize::Error::custom(
			"expected start object or null",
		)),
	}
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_tivet_api_kv_common_key_components<'a, I>(
	tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<std::string::String>>, aws_smithy_json::deserialize::Error>
where
	I: Iterator<
		Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
	>,
{
	match tokens.next().transpose()? {
		Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
		Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
			let mut items = Vec::new();
			loop {
				match tokens.peek() {
					Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
						tokens.next().transpose().unwrap();
						break;
					}
					_ => {
						let value = aws_smithy_json::deserialize::token::expect_string_or_null(
							tokens.next(),
						)?
						.map(|s| s.to_unescaped().map(|u| u.into_owned()))
						.transpose()?;
						if let Some(value) = value {
							items.push(value);
						}
					}
				}
			}
			Ok(Some(items))
		}
		_ => Err(aws_smithy_json::deserialize::Error::custom(
			"expected start array or null",
		)),
	}
}
