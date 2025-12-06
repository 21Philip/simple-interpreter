use std::collections::HashMap;

use crate::language::{Aexpr, Atomic, Statement};

#[derive(Debug)]
pub enum EvalError {
    UndeclaredVar(String),
}

fn atomic_eval(atom: Atomic, vars: &mut HashMap<String, i64>) -> Result<i64, EvalError> {
    match atom {
        Atomic::Int(i) => Ok(i),
        Atomic::Identifier(id) => vars.get(&id).copied().ok_or(EvalError::UndeclaredVar(id)),
    }
}

fn arith_eval(expr: Aexpr, vars: &mut HashMap<String, i64>) -> Result<i64, EvalError> {
    match expr {
        Aexpr::Add(a, b) => {
            let left = arith_eval(*a, vars)?;
            let right = arith_eval(*b, vars)?;
            Ok(left + right)
        }
        Aexpr::Sub(a, b) => {
            let left = arith_eval(*a, vars)?;
            let right = arith_eval(*b, vars)?;
            Ok(left - right)
        }
        Aexpr::Mul(a, b) => {
            let left = arith_eval(*a, vars)?;
            let right = arith_eval(*b, vars)?;
            Ok(left * right)
        }
        Aexpr::Div(a, b) => {
            let left = arith_eval(*a, vars)?;
            let right = arith_eval(*b, vars)?;
            Ok(left / right)
        }
        Aexpr::Negate(a) => arith_eval(*a, vars).map(|v| -v),
        Aexpr::Num(a) => atomic_eval(a, vars),
    }
}

pub fn statement_eval(stmnt: Statement, vars: &mut HashMap<String, i64>) -> Result<(), EvalError> {
    match stmnt {
        Statement::Sequence(mut statements) => {
            while let Some(next) = statements.pop_front() {
                statement_eval(next, vars)?;
            }
            Ok(())
        }
        Statement::Assign(id, value) => {
            let tmp = arith_eval(value, vars)?;
            vars.insert(id, tmp);
            Ok(())
        }
        Statement::Print(value) => {
            let tmp = arith_eval(value, vars)?;
            println!("{}", tmp);
            Ok(())
        }
    }
}
