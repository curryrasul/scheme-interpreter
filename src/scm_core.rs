use core::fmt;
use crate::util::NamedArgsList;

#[derive(Clone)]
pub enum ScmValue {
    Integer(i64),
    Number(f64), // TODO: exact/inexact
    Bool(bool),
    Char(char),
    String(String),
    Symbol(String),
    DotPair { car: Box<ScmValue>, cdr: Box<ScmValue> },
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
pub enum ScmProcUnit { // Used only as element of procedure's stack
    Val(ScmValue),
    Variable(String),
    ProcCall(ScmCallable, usize),
    Lambda { args: Vec<String>, units_cnt: usize },
}

#[derive(Clone)]
pub struct ScmVariablesSet {
    sets: Vec<NamedArgsList<ScmValue>>,
}

pub struct ScmExecContext {
    variables: ScmVariablesSet,
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

pub fn exec_callable(ctx: &ScmExecContext, proc: ScmCallable, call_args: &[ScmValue]) -> ScmValue {
    let res = match proc {
        ScmCallable::Builtin(func) => (func)(ctx, call_args),

        ScmCallable::CustomProc(p) => {
            let mut stack = Vec::<ScmValue>::new();

            let mut iter = p.instructions.iter().rev();
            while let Some(proc_unit) = iter.next() {
                match proc_unit {
                    ScmProcUnit::Val(v) => {
                        stack.push(v.clone());
                    },

                    ScmProcUnit::Variable(name) => {
                        let mut var = find_arg(name, call_args, &p.params);
                        if var.is_none() {
                            var = ctx.variables.find_var(&name);
                        }
                        assert!(!var.is_none(), "Unknown variable: {}", name);
                        stack.push(var.unwrap());
                    },

                    ScmProcUnit::ProcCall(proc, args_cnt) => {
                        let mut args = Vec::<ScmValue>::new();
                        for _ in 0..*args_cnt {
                            args.push(stack.pop().unwrap())
                        }

                        let res = match proc {
                            ScmCallable::Builtin(func) =>
                                (func)(ctx, &args),
                            ScmCallable::CustomProc(proc) =>
                                exec_callable(ctx, ScmCallable::CustomProc(proc.clone()), &args),
                        };

                        stack.push(res);
                    },

                    ScmProcUnit::Lambda { args, units_cnt } => {
                        let mut data = Vec::<ScmProcUnit>::new();
                        for _ in 0..*units_cnt {
                            let unit = iter.next().unwrap();
                            let val = match unit {
                                ScmProcUnit::Variable(name) => {
                                    let mut is_local_arg = false;
                                    for arg_name in args.iter() {
                                        if name == arg_name {
                                            is_local_arg = true;
                                            break;
                                        }
                                    }
                                    
                                    if is_local_arg {
                                        unit.clone()
                                    }
                                    else {
                                        let mut var = find_arg(name, call_args, &p.params);
                                        if var.is_none() {
                                            var = ctx.variables.find_var(&name);
                                        }
                                        assert!(!var.is_none(), "Unknown variable: {}", name);
                                        ScmProcUnit::Val(var.unwrap())
                                    }
                                },
                                _ => unit.clone(),
                            };
                            data.push(val);
                        }
                        
                        data.reverse();
                        let callable_lambda = ScmCallable::CustomProc(ScmProcedure {
                            params: args.clone(),
                            instructions: data,
                        });
                        stack.push(ScmValue::Procedure(callable_lambda))
                    },
                }
            }

            stack.pop().unwrap()
        }
    };

    return res;
}

impl ScmVariablesSet {
    pub fn find_var(&self, name: &String) -> Option<ScmValue> {
        for container in self.sets.iter().rev() {
            let val = container.find_by_name(name);
            if val.is_some() {
                return val;
            }
        }
        // panic!("Unknown variable: {}", name);
        return None
    }

    pub fn add_set(&mut self, container: NamedArgsList<ScmValue>) {
        self.sets.push(container);
    }

    pub fn pop_set(&mut self) {
        self.sets.pop();
    }
}

impl ScmExecContext {
    pub fn new() -> ScmExecContext {
        return ScmExecContext {
            variables: ScmVariablesSet {
                sets: Vec::new(),
            }
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
            ScmValue::DotPair { car, cdr } =>
                write!(f, "({} . {})", car, cdr),
            ScmValue::Nil => write!(f, "nil"),
            ScmValue::Procedure(_) => write!(f, "<proc>"),
        }
    }
}
