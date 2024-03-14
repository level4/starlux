extern crate starlark;

mod lark;
use lark::evaluate_starlark_code;
use std::env;
use anyhow::Result;
use rustler::{Encoder, Error, Env, NifResult, Term};
use serde_json::json;
use serde_json::Value as JsonValue;
mod atoms {
    rustler::atoms! {
        ok,
        error,
        nil
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn evaluate_and_return_json<'a>(env: Env<'a>, code: String) -> NifResult<Term<'a>> {
    match evaluate_starlark_code(&code) {
        Ok(json_output) => {
            // Deserialize the JSON output to a serde_json::Value
            let json: JsonValue = serde_json::from_str(&json_output)
                .map_err(|e| Error::RaiseTerm(Box::new(e.to_string())))?;

            // Convert the serde_json::Value into an Elixir term
            let term_result = serde_transcode::transcode(
                &mut serde_json::Deserializer::from_slice(json_output.as_bytes()),
                serde_rustler::Serializer::from(env)
            ).map_err(|e| Error::RaiseTerm(Box::new(format!("{}", e))))?;

            // Wrap the result in an :ok tuple
            let ok_atom = atoms::ok(); // Ensure you have defined this atom
            Ok((ok_atom, term_result).encode(env))
        },
        Err(e) => {
            // Wrap the error in an :error tuple
            let error_atom = atoms::error(); // Ensure you have defined this atom
            Ok((error_atom, e.to_string()).encode(env))
        },
    }
}


rustler::init!("Elixir.Starlux.Run", [evaluate_and_return_json]);
