use can::env::Env;
use can::expr::Expr;
use can::problem::Problem;
use can::problem::RuntimeError::*;
use constrain;
use region::Region;
use std::i64;
use subs::Subs;
use types::Constraint::{self, *};
use types::Expected;
use types::Type;

#[inline(always)]
pub fn int_expr_from_result(
    subs: &mut Subs,
    result: Result<i64, &str>,
    env: &mut Env,
    expected: Expected<Type>,
    region: Region,
) -> (Constraint, Expr) {
    match result {
        Ok(int) => (
            constrain::int_literal(subs, expected, region),
            Expr::Int(int),
        ),
        Err(raw) => {
            let runtime_error = IntOutsideRange(raw.into());

            env.problem(Problem::RuntimeError(runtime_error.clone()));

            (True, Expr::RuntimeError(runtime_error))
        }
    }
}

#[inline(always)]
pub fn float_expr_from_result(
    subs: &mut Subs,
    result: Result<f64, &str>,
    env: &mut Env,
    expected: Expected<Type>,
    region: Region,
) -> (Constraint, Expr) {
    match result {
        Ok(float) => (
            constrain::float_literal(subs, expected, region),
            Expr::Float(float),
        ),
        Err(raw) => {
            let runtime_error = FloatOutsideRange(raw.into());

            env.problem(Problem::RuntimeError(runtime_error.clone()));

            (True, Expr::RuntimeError(runtime_error))
        }
    }
}

#[inline(always)]
pub fn finish_parsing_int(raw: &str) -> Result<i64, &str> {
    // Ignore underscores.
    raw.replace("_", "").parse::<i64>().map_err(|_| raw)
}

#[inline(always)]
pub fn finish_parsing_hex(raw: &str) -> Result<i64, &str> {
    // Ignore underscores.
    i64::from_str_radix(raw.replace("_", "").as_str(), 16).map_err(|_| raw)
}

#[inline(always)]
pub fn finish_parsing_oct(raw: &str) -> Result<i64, &str> {
    // Ignore underscores.
    i64::from_str_radix(raw.replace("_", "").as_str(), 8).map_err(|_| raw)
}

#[inline(always)]
pub fn finish_parsing_bin(raw: &str) -> Result<i64, &str> {
    // Ignore underscores.
    i64::from_str_radix(raw.replace("_", "").as_str(), 2).map_err(|_| raw)
}

#[inline(always)]
pub fn finish_parsing_float(raw: &str) -> Result<f64, &str> {
    // Ignore underscores.
    match raw.replace("_", "").parse::<f64>() {
        Ok(float) if float.is_finite() => Ok(float),
        _ => Err(raw),
    }
}
