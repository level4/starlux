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
fn evaluate<'a>(env: Env<'a>, code: String) -> NifResult<Term<'a>> {
    match evaluate_starlark_code(&code) {
        Ok((eval_str, store_json)) => {
            // Convert the direct evaluation result to an Elixir term
            let eval_term = eval_str.encode(env);

            // Deserialize the store's JSON output to a serde_json::Value
            let store_json_value: JsonValue = serde_json::from_str(&store_json)
                .map_err(|e| Error::RaiseTerm(Box::new(e.to_string())))?;

            // Convert the serde_json::Value of the store into an Elixir term
            let store_term = serde_rustler::to_term(env, &store_json_value)
                .map_err(|e| Error::RaiseTerm(Box::new(format!("{}", e))))?;

            // Wrap the results in an :ok tuple along with both terms
            let ok_atom = atoms::ok(); // Ensure you have defined this atom
            Ok((ok_atom, (eval_term, store_term)).encode(env))
        },
        Err(e) => {
            // Wrap the error in an :error tuple
            let error_atom = atoms::error(); // Ensure you have defined this atom
            Ok((error_atom, e.to_string()).encode(env))
        },
    }
}


rustler::init!("Elixir.Starlux.Run", [evaluate]);
