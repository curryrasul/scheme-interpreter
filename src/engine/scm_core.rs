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
    DotPair {
        car: Box<ScmValue>,
        cdr: Box<ScmValue>,
    },
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

#[derive(Debug, Clone)]
pub enum ScmProcUnit {
    // Used only as element of procedure's stack
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

fn exec_custom_proc(
    ctx: &mut ScmExecContext,
    proc: ScmProcedure,
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
                let var = find_arg(name, call_args, &proc.params)
                    .or_else(|| ctx.variables.find_var(&name));
                assert!(!var.is_none(), "Unknown variable: {}", name);
                stack.push(var.unwrap());
            }

            ScmProcUnit::ProcCall(proc_name, args_cnt) => {
                let mut args = Vec::<ScmValue>::new();
                for _ in 0..*args_cnt {
                    args.push(stack.pop().unwrap())
                }

                let var = find_arg(proc_name, call_args, &proc.params)
                    .or_else(|| ctx.variables.find_var(&proc_name));
                assert!(!var.is_none(), "Unknown procedure: {}", proc_name);

                if let ScmValue::Procedure(proc) = var.unwrap() {
                    let res = match proc {
                        ScmCallable::Builtin(func) => (func)(ctx, &args),
                        ScmCallable::CustomProc(proc) => {
                            exec_callable(ctx, ScmCallable::CustomProc(proc.clone()), &args)
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
                                let var = find_arg(name, call_args, &proc.params)
                                    .or_else(|| ctx.variables.find_var(&name));
                                // assert!(!var.is_none(), "Unknown variable: {}", name);
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
    proc: ScmCallable,
    call_args: &[ScmValue],
) -> ScmValue {
    return match proc {
        ScmCallable::Builtin(func) => (func)(ctx, call_args),
        ScmCallable::CustomProc(proc) => exec_custom_proc(ctx, proc, call_args),
    };
}

impl ScmExecContext {
    pub fn new() -> ScmExecContext {
        let mut ctx = Self {
            variables: VariablesSet::new(),
        };

        ctx.add_or_assign_var("+", SCM_BUILTIN_ADD);
        ctx.add_or_assign_var("-", SCM_BUILTIN_SUB);
        ctx.add_or_assign_var("*", SCM_BUILTIN_MUL);
        ctx.add_or_assign_var("/", SCM_BUILTIN_DIV);
        ctx.add_or_assign_var("<", SCM_BUILTIN_LE);
        ctx.add_or_assign_var("=", SCM_BUILTIN_EQ);
        ctx.add_or_assign_var("newline", SCM_BUILTIN_NEWLINE);
        ctx.add_or_assign_var("display", SCM_BUILTIN_DISPLAY);
        ctx.add_or_assign_var("list", SCM_BUILTIN_LIST);
        ctx.add_or_assign_var("apply", SCM_BUILTIN_APPLY);
        ctx.add_or_assign_var("cons", SCM_BUILTIN_CONS);
        ctx.add_or_assign_var("car", SCM_BUILTIN_CAR);
        ctx.add_or_assign_var("cdr", SCM_BUILTIN_CDR);

        ctx
    }

    pub fn add_or_assign_var(&mut self, name: &str, val: ScmValue) {
        self.variables.add_or_assign_var(name, val);
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
            ScmValue::DotPair { car, cdr } => write!(f, "({:?} . {:?})", car, cdr),
            ScmValue::Nil => write!(f, "nil"),
            ScmValue::Procedure(_) => write!(f, "<proc>"),
        }
    }
}
