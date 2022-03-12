use crate::{
    engine::{scm_builtins::*, scm_is_true},
    VariablesSet,
};
use core::fmt;

#[derive(Clone)]
pub enum ScmValue {
    Integer(i64),
    Number(f64), // TODO: exact/inexact
    Bool(bool),
    Char(char),
    String(String),
    Symbol(String),
    DotPair(Box<ScmValue>, Box<ScmValue>),
    Nil,
    Procedure(ScmCallable),
}

#[derive(Clone)]
pub enum ScmCallable {
    Builtin(fn(ctx: &mut ScmExecContext, args: &[ScmValue]) -> ScmValue),
    CustomProc(ScmProcedure),
}

#[derive(Clone)]
pub struct ScmProcedure {
    pub params: Vec<String>,
    pub instructions: Vec<ScmProcUnit>,
}

// Used only as element of procedure's stack
#[derive(Debug, Clone)]
pub enum ScmProcUnit {
    Val(ScmValue),
    Variable(String),
    ProcCall(String, usize), // Name and args cnt
    Lambda { args: Vec<String>, units_cnt: usize },
    TrueBranch(usize),  // Skip size
    FalseBranch(usize), // Skip size
    Assign(String),     // Define and assign are same here
}

pub struct ScmExecContext {
    pub variables: VariablesSet<ScmValue>,
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

fn find_var(
    name: &String,
    args: &[ScmValue],
    args_names: &[String],
    ctx: &mut ScmExecContext,
) -> Option<ScmValue> {
    find_arg(name, args, args_names).or_else(|| ctx.variables.find_var(&name))
}

fn exec_custom_proc(
    ctx: &mut ScmExecContext,
    proc: &ScmProcedure,
    call_args: &[ScmValue],
) -> ScmValue {
    let mut stack = Vec::<ScmValue>::new();
    let mut iter = proc.instructions.iter().rev();

    while let Some(proc_unit) = iter.next() {
        match proc_unit {
            ScmProcUnit::Val(v) => {
                stack.push(v.clone());
            }

            ScmProcUnit::Variable(name) => {
                let var = find_var(name, call_args, &proc.params, ctx);
                assert!(!var.is_none(), "Unknown variable: {}", name);
                stack.push(var.unwrap());
            }

            ScmProcUnit::ProcCall(proc_name, args_cnt) => {
                let mut args = Vec::<ScmValue>::new();
                for _ in 0..*args_cnt {
                    args.push(stack.pop().unwrap())
                }

                let var = find_var(proc_name, call_args, &proc.params, ctx);
                assert!(!var.is_none(), "Unknown procedure: {}", proc_name);

                if let ScmValue::Procedure(proc) = var.unwrap() {
                    let res = match proc {
                        ScmCallable::Builtin(func) => (func)(ctx, &args),
                        ScmCallable::CustomProc(proc) => {
                            exec_callable(ctx, &ScmCallable::CustomProc(proc), &args)
                        }
                    };

                    stack.push(res);
                } else {
                    panic!("Variable cannot be called: {}", proc_name);
                }
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
                                let var = find_var(name, call_args, &proc.params, ctx);
                                match var {
                                    None => unit.clone(),
                                    Some(_) => ScmProcUnit::Val(var.unwrap()),
                                }
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

            ScmProcUnit::TrueBranch(skip_cnt) => {
                let cond = stack.pop().unwrap();
                if !scm_is_true(&cond) {
                    for _ in 0..*skip_cnt {
                        iter.next();
                    }
                }
            }

            ScmProcUnit::FalseBranch(skip_cnt) => {
                iter.next();
                for _ in 0..*skip_cnt {
                    iter.next();
                }
            }

            ScmProcUnit::Assign(name) => {
                let val = stack.pop().unwrap();
                ctx.variables.add_or_assign_var(name, val);
            }
        }
    }

    stack.pop().unwrap()
}

pub fn exec_callable(
    ctx: &mut ScmExecContext,
    proc: &ScmCallable,
    call_args: &[ScmValue],
) -> ScmValue {
    return match proc {
        ScmCallable::Builtin(func) => (func)(ctx, call_args),
        ScmCallable::CustomProc(proc) => exec_custom_proc(ctx, &proc, call_args),
    };
}

impl ScmExecContext {
    pub fn new() -> ScmExecContext {
        let mut ctx = Self {
            variables: VariablesSet::new(),
        };
        for builtin in BUILTINS_LIST.iter() {
            ctx.add_or_assign_var(builtin.0, builtin.1.clone());
        }
        ctx
    }

    pub fn add_or_assign_var(&mut self, name: &str, val: ScmValue) {
        self.variables.add_or_assign_var(name, val);
    }
}

impl fmt::Display for ScmProcedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instr in self.instructions.iter() {
            writeln!(f, "{:?}", instr)?;
        }
        Result::Ok(())
    }
}

impl fmt::Debug for ScmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScmValue::Integer(val) => write!(f, "ScmValue::Integer({})", val),
            ScmValue::Number(val) => write!(f, "ScmValue::Number({})", val),
            ScmValue::Bool(val) => write!(f, "ScmValue::Bool({})", val),
            ScmValue::Char(val) => write!(f, "ScmValue::Char({})", val),
            ScmValue::String(val) => write!(f, "ScmValue::String({})", val),
            ScmValue::Symbol(val) => write!(f, "ScmValue::Symbol({})", val),
            ScmValue::DotPair(car, cdr) => write!(f, "({:?} . {:?})", car, cdr),
            ScmValue::Nil => write!(f, "nil"),
            ScmValue::Procedure(_) => write!(f, "<proc>"),
        }
    }
}
