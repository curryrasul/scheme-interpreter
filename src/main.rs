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

    ctx.variables.add_or_assign_var(&String::from("+"), ScmValue::Procedure(SCM_BUILTIN_ADD));
    ctx.variables.add_or_assign_var(&String::from("newline"), ScmValue::Procedure(SCM_BUILTIN_NEWLINE));
    ctx.variables.add_or_assign_var(&String::from("display"), ScmValue::Procedure(SCM_BUILTIN_DISPLAY));
    ctx.variables.add_or_assign_var(&String::from("list"), ScmValue::Procedure(SCM_BUILTIN_LIST));
    ctx.variables.add_or_assign_var(&String::from("apply"), ScmValue::Procedure(SCM_BUILTIN_APPLY));
    ctx.variables.add_or_assign_var(&String::from("cons"), ScmValue::Procedure(SCM_BUILTIN_CONS));
    ctx.variables.add_or_assign_var(&String::from("car"), ScmValue::Procedure(SCM_BUILTIN_CAR));
    ctx.variables.add_or_assign_var(&String::from("cdr"), ScmValue::Procedure(SCM_BUILTIN_CDR));

    let proc = gen_test_code3();

    let callable = ScmCallable::CustomProc(proc);
    exec_callable(&mut ctx, callable, &Vec::new());
}
