use core::fmt;

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
    Builtin(fn(args: &[ScmValue]) -> ScmValue),
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
    Param(usize), // Index of param
    Proc(ScmCallable, usize),
}

pub fn exec_callable(proc: ScmCallable, args: &[ScmValue]) -> ScmValue {
    return match proc {
        ScmCallable::Builtin(func) => (func)(args),
        ScmCallable::CustomProc(p) => {
            let mut stack = Vec::<ScmValue>::new();

            for proc_unit in p.instructions.iter().rev() {
                match proc_unit {
                    ScmProcUnit::Val(v) => stack.push(v.clone()),
                    ScmProcUnit::Param(idx) => stack.push(args[*idx].clone()),
                    ScmProcUnit::Proc(proc, args_cnt) => {
                        let mut args = Vec::<ScmValue>::new();
                        for _ in 0..*args_cnt {
                            args.push(stack.pop().unwrap())
                        }

                        let res = match proc {
                            ScmCallable::Builtin(func) =>
                                (func)(&args),
                            ScmCallable::CustomProc(proc) =>
                                exec_callable(ScmCallable::CustomProc(proc.clone()), &args),
                        };

                        stack.push(res)
                    }
                }
            }

            stack.pop().unwrap() 
        }
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
