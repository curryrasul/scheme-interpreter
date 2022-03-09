use project::*;

#[allow(dead_code)]
fn gen_test_code1() -> ScmProcedure {
    let mut instr = Vec::<ScmProcUnit>::new();

    // (+ 3 (+ 4 1 (cdr (cons -100 100))))
    instr.push(ScmProcUnit::ProcCall(String::from("newline"), 0));
    instr.push(ScmProcUnit::ProcCall(String::from("display"), 1));
    instr.push(ScmProcUnit::ProcCall(String::from("+"), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(3)));
    instr.push(ScmProcUnit::ProcCall(String::from("+"), 3));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(4)));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(1)));
    instr.push(ScmProcUnit::ProcCall(String::from("cdr"), 1));
    instr.push(ScmProcUnit::ProcCall(String::from("cons"), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(-100)));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(100)));

    return ScmProcedure {
        params: Vec::<String>::new(),
        instructions: instr,
    };
}

#[allow(dead_code)]
fn gen_test_code2() -> ScmProcedure {
    let mut instr = Vec::<ScmProcUnit>::new();

    // (apply (lambda (x y) (+ x y 5)) (cons 3 (cons 7 ())) )
    instr.push(ScmProcUnit::ProcCall(String::from("newline"), 0));
    instr.push(ScmProcUnit::ProcCall(String::from("display"), 1));

    instr.push(ScmProcUnit::ProcCall(String::from("apply"), 2));
    instr.push(ScmProcUnit::ProcCall(String::from("+"), 3));
    instr.push(ScmProcUnit::Variable(String::from("x")));
    instr.push(ScmProcUnit::Variable(String::from("y")));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(5)));
    instr.push(ScmProcUnit::Lambda {
        args: vec![String::from("x"), String::from("y")],
        units_cnt: 4,
    });

    instr.push(ScmProcUnit::ProcCall(String::from("list"), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(3)));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(7)));

    return ScmProcedure {
        params: Vec::<String>::new(),
        instructions: instr,
    };
}

#[allow(dead_code)]
fn gen_test_code3() -> ScmProcedure {
    let mut instr = Vec::<ScmProcUnit>::new();

    // (begin
    //     (if #f
    //         (begin (display 55) (newline))
    //         (begin (display 10) (newline))
    //     (display 55) (newline))

    instr.push(ScmProcUnit::ProcCall(String::from("newline"), 0));
    instr.push(ScmProcUnit::ProcCall(String::from("display"), 1));
    instr.push(ScmProcUnit::Variable(String::from("myvar")));

    instr.push(ScmProcUnit::ProcCall(String::from("newline"), 0));
    instr.push(ScmProcUnit::ProcCall(String::from("display"), 1));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(10)));
    instr.push(ScmProcUnit::FalseBranch(3));

    instr.push(ScmProcUnit::ProcCall(String::from("newline"), 0));
    instr.push(ScmProcUnit::ProcCall(String::from("display"), 1));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(55)));
    instr.push(ScmProcUnit::TrueBranch(4));

    instr.push(ScmProcUnit::Val(ScmValue::Bool(false)));

    instr.push(ScmProcUnit::Assign(
        String::from("myvar"),
        ScmValue::Integer(666),
    ));

    return ScmProcedure {
        params: Vec::<String>::new(),
        instructions: instr,
    };
}

fn main() {
    let mut ctx = ScmExecContext::new();

    ctx.add_or_assign_var("+", SCM_BUILTIN_ADD);
    ctx.add_or_assign_var("newline", SCM_BUILTIN_NEWLINE);
    ctx.add_or_assign_var("display", SCM_BUILTIN_DISPLAY);
    ctx.add_or_assign_var("list", SCM_BUILTIN_LIST);
    ctx.add_or_assign_var("apply", SCM_BUILTIN_APPLY);
    ctx.add_or_assign_var("cons", SCM_BUILTIN_CONS);
    ctx.add_or_assign_var("car", SCM_BUILTIN_CAR);
    ctx.add_or_assign_var("cdr", SCM_BUILTIN_CDR);

    let mut parser = Parser::new("(display (+ 1 2))");
    let callable = parser.parse();
    // let callable = ScmCallable::CustomProc(gen_test_code3());
    exec_callable(&mut ctx, callable, &Vec::new());
}
