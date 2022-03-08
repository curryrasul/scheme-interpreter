use project::*;

#[allow(dead_code)]
fn gen_test_code1() -> ScmProcedure {
    let mut instr = Vec::<ScmProcUnit>::new();

    // (+ 3 (+ 4 1 (cdr (cons -100 100))))
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

    // (apply (lambda (x y) (+ x y 5)))
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_APPLY.clone(), 2));
    
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_ADD.clone(), 3));
    instr.push(ScmProcUnit::Variable(String::from("x")));
    instr.push(ScmProcUnit::Variable(String::from("y")));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(5)));
    instr.push(ScmProcUnit::Lambda{ args: vec![ String::from("x"), String::from("y") ], units_cnt: 4 });

    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_CONS.clone(), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(3)));
    instr.push(ScmProcUnit::ProcCall(SCM_BUILTIN_CONS.clone(), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(7)));
    instr.push(ScmProcUnit::Val(ScmValue::Nil));

    return ScmProcedure {
        params: Vec::<String>::new(),
        instructions: instr,
    };
}

fn main() {
    let ctx = ScmExecContext::new();
    let proc = gen_test_code2();

    let callable = ScmCallable::CustomProc(proc);
    let res = exec_callable(&ctx, callable, &Vec::new());
    println!("{}", res);


    // let test_prog = "(add 5 (car (cons 2 4)))";
    
}
