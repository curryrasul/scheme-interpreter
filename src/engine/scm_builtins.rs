use crate::{
    scm_core::*, scm_list_len, scm_list_to_vec, scm_utils::scm_is_list, typed_num::TypedNum,
};

macro_rules! scm_builtin_impl {
    ($name:expr,$func:expr) => {
        ($name, ScmValue::Procedure(ScmCallable::Builtin($func)))
    };
}

pub const BUILTINS_LIST: &[(&str, ScmValue)] = &[
    //
    // System
    //
    scm_builtin_impl!("apply", |ctx, args| -> ScmValue {
        assert!(args.len() == 2);
        assert!(scm_is_list(&args[1]));

        let proc = match &args[0] {
            ScmValue::Procedure(proc) => proc,
            _ => {
                panic!("Only procedures can be called");
            }
        };

        let call_args = &scm_list_to_vec(&args[1]);

        exec_callable(ctx, proc, call_args)
    }),
    scm_builtin_impl!("display", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        print!("{:?}", args[0]);
        ScmValue::Nil
    }),
    scm_builtin_impl!("error", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        eprint!("{:?}", args[0]);
        ScmValue::Nil
    }),
    scm_builtin_impl!("newline", |_, args| -> ScmValue {
        assert!(args.is_empty());
        println!("");
        ScmValue::Nil
    }),
    //
    // Arithmetics
    //
    scm_builtin_impl!("+", |_, args| -> ScmValue {
        if args.is_empty() {
            ScmValue::Number(TypedNum::Integer(0))
        } else {
            let mut res = TypedNum::Integer(0);
            for arg in args.iter() {
                if let ScmValue::Number(val) = arg {
                    res = res + *val;
                } else {
                    panic!("Adding non numeric values");
                }
            }
            ScmValue::Number(res)
        }
    }),
    scm_builtin_impl!("-", |_, args| -> ScmValue {
        if args.is_empty() {
            return ScmValue::Number(TypedNum::Integer(0)); // TODO
        }

        if args.len() == 1 {
            if let ScmValue::Number(n) = args[0] {
                return ScmValue::Number(-n);
            } else {
                panic!("Unsupported value");
            }
        }

        let mut res = if let ScmValue::Number(n) = args[0] {
            n
        } else {
            panic!("Unsupported value");
        };

        for arg in args.iter().skip(1) {
            if let ScmValue::Number(n) = arg {
                res = res - *n;
            } else {
                panic!("Unsupported value");
            }
        }

        ScmValue::Number(res)
    }),
    scm_builtin_impl!("*", |_, args| -> ScmValue {
        if args.is_empty() {
            ScmValue::Number(TypedNum::Integer(1))
        } else {
            let mut res = TypedNum::Integer(0);
            for arg in args.iter() {
                if let ScmValue::Number(val) = arg {
                    res = res * *val;
                } else {
                    panic!("Multiplying non numeric values");
                }
            }
            ScmValue::Number(res)
        }
    }),
    scm_builtin_impl!("/", |_, args| -> ScmValue {
        if args.is_empty() {
            return ScmValue::Number(TypedNum::Integer(0)); // TODO
        }

        if args.len() == 1 {
            if let ScmValue::Number(n) = args[0] {
                return ScmValue::Number(TypedNum::Float(1f64) / n);
            } else {
                panic!("Unsupported value");
            }
        }

        let numer = if let ScmValue::Number(n) = args[0] {
            n
        } else {
            panic!("Unsupported value");
        };

        let mut denom = TypedNum::Float(1f64);
        for arg in args.iter().skip(1) {
            if let ScmValue::Number(n) = arg {
                denom = denom * *n;
            } else {
                panic!("Unsupported value");
            }
        }

        ScmValue::Number(numer / denom)
    }),
    scm_builtin_impl!("abs", |_, args| -> ScmValue {
        assert!(args.len() == 1, "ABS requires exactly 1 argument");
        if let ScmValue::Number(val) = args[0] {
            ScmValue::Number(val.abs())
        } else {
            panic!("ABS requires numeric argument");
        }
    }),
    //
    // Comparison
    //
    scm_builtin_impl!("=", |_, args| -> ScmValue {
        assert!(args.len() == 2);
        if let ScmValue::Number(v1) = args[0] {
            if let ScmValue::Number(v2) = args[1] {
                ScmValue::Bool(v1 == v2)
            } else {
                panic!("");
            }
        } else {
            panic!("");
        }
    }),
    scm_builtin_impl!("<", |_, args| -> ScmValue {
        assert!(args.len() == 2, "LT requires exactly 2 arguments");

        if let ScmValue::Number(v1) = args[0] {
            if let ScmValue::Number(v2) = args[1] {
                ScmValue::Bool(v1 < v2)
            } else {
                panic!("");
            }
        } else {
            panic!("");
        }
    }),
    //
    // Pairs and lists
    //
    scm_builtin_impl!("cons", |_, args| -> ScmValue {
        assert!(args.len() == 2, "CONS requires exactly 2 arguments");
        ScmValue::DotPair(Box::new(args[0].clone()), Box::new(args[1].clone()))
    }),
    scm_builtin_impl!("car", |_, args| -> ScmValue {
        assert!(args.len() == 1, "CAR requires exactly 1 argument");
        return match args[0].clone() {
            ScmValue::DotPair(car, _) => (*car).clone(),
            _ => {
                panic!("Car requires argument of type DotPair");
            }
        };
    }),
    scm_builtin_impl!("cdr", |_, args| -> ScmValue {
        assert!(args.len() == 1, "CDR requires exactly 1 argument");
        return match args[0].clone() {
            ScmValue::DotPair(cdr, _) => (*cdr).clone(),
            _ => {
                panic!("Car requires argument of type DotPair");
            }
        };
    }),
    scm_builtin_impl!("list", |_, args| -> ScmValue {
        let mut res = ScmValue::Nil;
        for arg in args.iter().rev() {
            res = ScmValue::DotPair(Box::new(arg.clone()), Box::new(res));
        }
        res
    }),
    scm_builtin_impl!("length", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        let res = scm_list_len(&args[0]);
        ScmValue::Number(TypedNum::Integer(res.unwrap()))
    }),
    //
    // Types predicates
    //
    scm_builtin_impl!("atom?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Bool(_)
        | ScmValue::Char(_)
        | ScmValue::Number(_)
        | ScmValue::String(_)
        | ScmValue::Symbol(_)
        | ScmValue::Nil = args[0]
        {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("bool?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Bool(_) = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("integer?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Number(num) = args[0] {
            ScmValue::Bool(matches!(num, TypedNum::Integer(_)))
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("number?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Number(_) = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("null?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Nil = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("pair?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::DotPair { .. } = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("list?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        ScmValue::Bool(scm_is_list(&args[0]))
    }),
    scm_builtin_impl!("procedure?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Procedure(_) = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("string?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::String(_) = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
    scm_builtin_impl!("symbol?", |_, args| -> ScmValue {
        assert!(args.len() == 1);
        if let ScmValue::Symbol(_) = args[0] {
            ScmValue::Bool(true)
        } else {
            ScmValue::Bool(false)
        }
    }),
];
