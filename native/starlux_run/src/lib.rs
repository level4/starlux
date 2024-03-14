extern crate starlark;

mod lark;
use lark::evaluate_starlark_code;
use std::env;
use anyhow::Result;

use rustler::{Encoder, Env, NifResult, Term};
use serde_json::json;

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}


#[rustler::nif]
fn evaluate_and_return_json(env: Env, code: String) -> NifResult<Term> {
    match evaluate_starlark_code(&code) {
        Ok(json_output) => Ok((atoms::ok(), json_output).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_string()).encode(env)),
    }
}

rustler::init!("Elixir.Starlux.Run", [evaluate_and_return_json]);
