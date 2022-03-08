use crate::{scm_core::*, scm_utils::scm_is_list, scm_list_to_vec};

pub const SCM_BUILTIN_APPLY: ScmCallable = ScmCallable::Builtin(builtin_apply);
pub const SCM_BUILTIN_ADD: ScmCallable = ScmCallable::Builtin(builtin_add);
pub const SCM_BUILTIN_CONS: ScmCallable = ScmCallable::Builtin(builtin_cons);
pub const SCM_BUILTIN_CAR: ScmCallable = ScmCallable::Builtin(builtin_car);
pub const SCM_BUILTIN_CDR: ScmCallable = ScmCallable::Builtin(builtin_cdr);
pub const SCM_BUILTIN_IS_LIST: ScmCallable = ScmCallable::Builtin(builtin_is_list);


fn builtin_apply(ctx: &ScmExecContext, args: &[ScmValue]) -> ScmValue {
    assert!(args.len() == 2);
    assert!(scm_is_list(&args[1]));

    let proc = match &args[0] {
        ScmValue::Procedure(proc) => proc,
        _ => { panic!("Only procedures can be called"); }
    };

    let call_args = &scm_list_to_vec(&args[1]);

    return exec_callable(ctx, proc.clone(), call_args);
}

fn builtin_add(_: &ScmExecContext, args: &[ScmValue]) -> ScmValue {
    if args.len() == 0 {
        return ScmValue::Integer(0);
    }
    else {
        let mut is_integer = true;
        for arg in args.iter() {
            match *arg {
                ScmValue::Integer(_) => {},
                ScmValue::Number(_) => {
                    is_integer = false;
                    break;
                },
                _ => { panic!("Adding non numeric values"); }
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
        }
        else {
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
}

fn builtin_cons(_: &ScmExecContext, args: &[ScmValue]) -> ScmValue {
    if args.len() != 2 {
        panic!("Cons requires exactly 2 arguments");
    }
    return ScmValue::DotPair {
        car: Box::new(args[0].clone()),
        cdr: Box::new(args[1].clone()),  
    };
}

fn builtin_car(_: &ScmExecContext, args: &[ScmValue]) -> ScmValue {
    if args.len() != 1 {
        panic!("Car requires exactly 1 argument");
    }
    return match args[0].clone() {
        ScmValue::DotPair{ car, .. } => (*car).clone(),
        _ => { panic!("Car requires argument of type DotPair"); },
    };
}

fn builtin_cdr(_: &ScmExecContext, args: &[ScmValue]) -> ScmValue {
    if args.len() != 1 {
        panic!("Car requires exactly 1 argument");
    }
    return match args[0].clone() {
        ScmValue::DotPair{ cdr, .. } => (*cdr).clone(),
        _ => { panic!("Car requires argument of type DotPair"); },
    };
}

fn builtin_is_list(_: &ScmExecContext, args: &[ScmValue]) -> ScmValue {
    assert!(args.len() == 1);
    return ScmValue::Bool(scm_is_list(&args[0]));
}
