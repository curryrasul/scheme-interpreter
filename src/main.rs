use project::*;

#[allow(dead_code)]
fn gen_test_code1() -> ScmProcedure {
    let mut instr = Vec::<ScmProcUnit>::new();

    // (+ 3 (+ 4 1 (cdr (cons -100 100))))
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_NEWLINE.clone(), 0));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_DISPLAY.clone(), 1));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_ADD.clone(), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(3)));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_ADD.clone(), 3));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(4)));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(1)));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_CDR.clone(), 1));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_CONS.clone(), 2));
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
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_NEWLINE.clone(), 0));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_DISPLAY.clone(), 1));

    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_APPLY.clone(), 2));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_ADD.clone(), 3));
    instr.push(ScmProcUnit::Variable(String::from("x")));
    instr.push(ScmProcUnit::Variable(String::from("y")));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(5)));
    instr.push(ScmProcUnit::Lambda {
        args: vec![String::from("x"), String::from("y")],
        units_cnt: 4,
    });

    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_LIST.clone(), 2));
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

    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_NEWLINE.clone(), 0));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_DISPLAY.clone(), 1));
    instr.push(ScmProcUnit::Variable(String::from("myvar")));

    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_NEWLINE.clone(), 0));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_DISPLAY.clone(), 1));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(10)));
    instr.push(ScmProcUnit::FalseBranch(3));

    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_NEWLINE.clone(), 0));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_DISPLAY.clone(), 1));
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
    let proc = gen_test_code3();

    let callable = ScmCallable::CustomProc(proc);
    exec_callable(&mut ctx, callable, &Vec::new());
}
