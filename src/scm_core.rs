use crate::{VariablesSet};
use core::fmt;

#[derive(Clone)]
pub enum ScmValue {
    Integer(i64),
    Number(f64), // TODO: exact/inexact
    Bool(bool),
    Char(char),
    String(String),
    Symbol(String),
    DotPair {
        car: Box<ScmValue>,
        cdr: Box<ScmValue>,
    },
    Nil,
    Procedure(ScmCallable),
}

#[derive(Clone)]
pub enum ScmCallable {
    Builtin(fn(ctx: &ScmExecContext, args: &[ScmValue]) -> ScmValue),
    CustomProc(ScmProcedure),
}

#[derive(Clone)]
pub struct ScmProcedure {
    pub params: Vec<String>,
    pub instructions: Vec<ScmProcUnit>,
}

#[derive(Clone)]
pub enum ScmProcUnit {
    // Used only as element of procedure's stack
    Val(ScmValue),
    Variable(String),
    ProcCall(ScmCallable, usize),
    Lambda { args: Vec<String>, units_cnt: usize },
}

pub struct ScmExecContext {
    variables: VariablesSet<ScmValue>,
}

fn find_arg(name: &String, args: &[ScmValue], args_names: &[String]) -> Option<ScmValue> {
    assert!(args.len() == args_names.len());
    let mut res = None;
    for i in 0..args_names.len() {
        if *name == args_names[i] {
            res = Some(args[i].clone());
            break;
        }
    }
    return res;
}

fn exec_custom_proc(ctx: &ScmExecContext, proc: ScmProcedure, call_args: &[ScmValue]) -> ScmValue {
    let mut stack = Vec::<ScmValue>::new();
    let mut iter = proc.instructions.iter().rev();

    while let Some(proc_unit) = iter.next() {
        match proc_unit {
            ScmProcUnit::Val(v) => {
                stack.push(v.clone());
            }

            ScmProcUnit::Variable(name) => {
                let var = find_arg(name, call_args, &proc.params)
                    .or_else(|| ctx.variables.find_var(&name));
                assert!(!var.is_none(), "Unknown variable: {}", name);
                stack.push(var.unwrap());
            }

            ScmProcUnit::ProcCall(proc, args_cnt) => {
                let mut args = Vec::<ScmValue>::new();
                for _ in 0..*args_cnt {
                    args.push(stack.pop().unwrap())
                }

                let res = match proc {
                    ScmCallable::Builtin(func) => (func)(ctx, &args),
                    ScmCallable::CustomProc(proc) => {
                        exec_callable(ctx, ScmCallable::CustomProc(proc.clone()), &args)
                    }
                };

                stack.push(res);
            }

            ScmProcUnit::Lambda { args, units_cnt } => {
                let mut data = Vec::<ScmProcUnit>::new();
                for _ in 0..*units_cnt {
                    let unit = iter.next().unwrap();
                    let val = match unit {
                        ScmProcUnit::Variable(name) => {
                            if args.iter().any(|it| name == it) {
                                unit.clone()
                            } else {
                                let var = find_arg(name, call_args, &proc.params)
                                    .or_else(|| ctx.variables.find_var(&name));
                                assert!(!var.is_none(), "Unknown variable: {}", name);
                                ScmProcUnit::Val(var.unwrap())
                            }
                        }

                        _ => unit.clone(),
                    };
                    data.push(val);
                }

                data.reverse();
                stack.push(ScmValue::Procedure(ScmCallable::CustomProc(ScmProcedure {
                    params: args.clone(),
                    instructions: data,
                })));
            }
        }
    }

    stack.pop().unwrap()
}

pub fn exec_callable(ctx: &ScmExecContext, proc: ScmCallable, call_args: &[ScmValue]) -> ScmValue {
    return match proc {
        ScmCallable::Builtin(func) => (func)(ctx, call_args),
        ScmCallable::CustomProc(proc) => exec_custom_proc(ctx, proc, call_args),
    };
}

impl ScmExecContext {
    pub fn new() -> ScmExecContext {
        return ScmExecContext {
            variables: VariablesSet::new(),
        };
    }
}

impl fmt::Display for ScmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScmValue::Integer(val) => write!(f, "{}", val),
            ScmValue::Number(val) => write!(f, "{}", val),
            ScmValue::Bool(val) => write!(f, "{}", val),
            ScmValue::Char(val) => write!(f, "{}", val),
            ScmValue::String(val) => write!(f, "{}", val),
            ScmValue::Symbol(val) => write!(f, "'{}", val),
            ScmValue::DotPair { car, cdr } => write!(f, "({} . {})", car, cdr),
            ScmValue::Nil => write!(f, "nil"),
            ScmValue::Procedure(_) => write!(f, "<proc>"),
        }
    }
}
