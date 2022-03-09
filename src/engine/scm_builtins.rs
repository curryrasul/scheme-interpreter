use crate::{scm_core::*, scm_list_len, scm_list_to_vec, scm_utils::scm_is_list};

// System

pub const SCM_BUILTIN_APPLY: ScmCallable = ScmCallable::Builtin(|ctx, args| -> ScmValue {
    assert!(args.len() == 2);
    assert!(scm_is_list(&args[1]));

    let proc = match &args[0] {
        ScmValue::Procedure(proc) => proc,
        _ => {
            panic!("Only procedures can be called");
        }
    };

    let call_args = &scm_list_to_vec(&args[1]);

    return exec_callable(ctx, proc.clone(), call_args);
});

pub const SCM_BUILTIN_DISPLAY: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    print!("{:?}", args[0]);
    return ScmValue::Nil;
});

pub const SCM_BUILTIN_ERROR: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    eprint!("{:?}", args[0]);
    return ScmValue::Nil;
});

pub const SCM_BUILTIN_NEWLINE: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 0);
    println!("");
    return ScmValue::Nil;
});

// Arithmetics

pub const SCM_BUILTIN_ADD: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    if args.len() == 0 {
        return ScmValue::Integer(0);
    } else {
        let mut is_integer = true;
        for arg in args.iter() {
            match *arg {
                ScmValue::Integer(_) => {}
                ScmValue::Number(_) => {
                    is_integer = false;
                    break;
                }
                _ => {
                    panic!("Adding non numeric values");
                }
            };
        }

        if is_integer {
            let mut sum = 0i64;
            for arg in args.iter() {
                sum += match *arg {
                    ScmValue::Integer(val) => val,
                    _ => panic!(""),
                }
            }
            return ScmValue::Integer(sum);
        } else {
            let mut sum = 0f64;
            for arg in args.iter() {
                sum += match *arg {
                    ScmValue::Integer(val) => val as f64,
                    ScmValue::Number(val) => val,
                    _ => panic!(""),
                }
            }
            return ScmValue::Number(sum);
        }
    }
});

pub const SCM_BUILTIN_SUB: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    if args.len() == 0 {
        return ScmValue::Integer(0);
    }

    if args.len() == 1 {
        if let ScmValue::Integer(n) = args[0] {
            return ScmValue::Integer(-n);
        }
    }

    for arg in args.iter() {
        match *arg {
            ScmValue::Integer(_) => (),
            _ => panic!("Unsupported value"),
        }
    }

    let mut iterator = args.iter();
    let mut sub = 0;

    if let ScmValue::Integer(n) = iterator.next().unwrap() {
        sub = *n;
    }

    for arg in iterator {
        if let ScmValue::Integer(n) = arg {
            sub -= *n;
        }
    }

    ScmValue::Integer(sub)
});

pub const SCM_BUILTIN_MUL: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    if args.len() == 0 {
        return ScmValue::Integer(0);
    }

    if args.len() == 1 {
        if let ScmValue::Integer(n) = args[0] {
            return ScmValue::Integer(n);
        }
    }

    for arg in args.iter() {
        match *arg {
            ScmValue::Integer(_) => (),
            _ => panic!("Unsupported value"),
        }
    }

    let mut mul = 1;
    for arg in args {
        if let ScmValue::Integer(n) = arg {
            mul *= *n;
        }
    }

    ScmValue::Integer(mul)
});

pub const SCM_BUILTIN_DIV: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    if args.len() == 0 {
        return ScmValue::Integer(0);
    }

    if args.len() == 1 {
        if let ScmValue::Integer(n) = args[0] {
            if n == 0 {
                panic!("Division by zero")
            }
            return ScmValue::Integer(1 / n);
        }
    }

    for arg in args.iter() {
        match *arg {
            ScmValue::Integer(_) => (),
            _ => panic!("Unsupported value"),
        }
    }

    let mut iterator = args.iter();
    let mut numerator = 1;
    let mut denominator = 1;

    if let ScmValue::Integer(n) = iterator.next().unwrap() {
        numerator = *n;
    }

    for arg in iterator {
        if let ScmValue::Integer(n) = arg {
            denominator *= *n;
        }
    }

    ScmValue::Integer(numerator / denominator)
});

pub const SCM_BUILTIN_ABS: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1, "ABS requires exactly 1 argument");
    return match args[0].clone() {
        ScmValue::Integer(val) => ScmValue::Integer(val.abs()),
        ScmValue::Number(val) => ScmValue::Number(val.abs()),
        _ => {
            panic!("ABS requires numeric argument");
        }
    };
});

// Pairs and lists

pub const SCM_BUILTIN_CONS: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 2, "CONS requires exactly 2 arguments");
    return ScmValue::DotPair {
        car: Box::new(args[0].clone()),
        cdr: Box::new(args[1].clone()),
    };
});

pub const SCM_BUILTIN_CAR: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1, "CAR requires exactly 1 argument");
    return match args[0].clone() {
        ScmValue::DotPair { car, .. } => (*car).clone(),
        _ => {
            panic!("Car requires argument of type DotPair");
        }
    };
});

pub const SCM_BUILTIN_CDR: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1, "CDR requires exactly 1 argument");
    return match args[0].clone() {
        ScmValue::DotPair { cdr, .. } => (*cdr).clone(),
        _ => {
            panic!("Car requires argument of type DotPair");
        }
    };
});

pub const SCM_BUILTIN_LIST: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    let mut res = ScmValue::Nil;
    for arg in args.iter().rev() {
        res = ScmValue::DotPair {
            car: Box::new(arg.clone()),
            cdr: Box::new(res),
        };
    }
    return res;
});

pub const SCM_BUILTIN_LENGTH: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    let res = scm_list_len(&args[0]);
    return ScmValue::Integer(res.unwrap());
});

// Types predicates

pub const SCM_BUILTIN_IS_ATOM: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Bool(_)
    | ScmValue::Char(_)
    | ScmValue::Integer(_)
    | ScmValue::Number(_)
    | ScmValue::String(_)
    | ScmValue::Symbol(_)
    | ScmValue::Nil = args[0]
    {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_BOOL: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Bool(_) = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_INTEGER: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Integer(_) = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_NUMBER: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Number(_) = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_NULL: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Nil = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_PAIR: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::DotPair { .. } = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_LIST: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    return ScmValue::Bool(scm_is_list(&args[0]));
});

pub const SCM_BUILTIN_IS_PROCEDURE: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Procedure(_) = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_STRING: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::String(_) = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});

pub const SCM_BUILTIN_IS_SYMBOL: ScmCallable = ScmCallable::Builtin(|_, args| -> ScmValue {
    assert!(args.len() == 1);
    if let ScmValue::Symbol(_) = args[0] {
        return ScmValue::Bool(true);
    } else {
        return ScmValue::Bool(false);
    };
});
