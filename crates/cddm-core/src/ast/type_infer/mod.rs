#![forbid(unsafe_code)]

pub mod signatures;
pub mod types;

#[cfg(test)]
mod tests;

pub use signatures::{
    format_call_site, format_function_signature, format_function_signature_with_return,
    to_pascal_case, to_snake_case,
};
pub use types::{default_type_for_ext, infer_parameter_type, infer_return_type};
