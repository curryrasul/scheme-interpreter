use project::*;

fn gen_test_code() -> ScmProcedure {
    let mut instr = Vec::<ScmProcUnit>::new();
 
    instr.push(ScmProcUnit::Proc(SCM_BUILTIN_ADD.clone(), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(3)));
    instr.push(ScmProcUnit::Proc(SCM_BUILTIN_ADD.clone(), 3));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(4)));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(1)));
    instr.push(ScmProcUnit::Proc(SCM_BUILTIN_CDR.clone(), 1));
    instr.push(ScmProcUnit::Proc(SCM_BUILTIN_CONS.clone(), 2));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(-100)));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(100)));

    return ScmProcedure {
        params: Vec::<String>::new(),
        instructions: instr,
    };
}

fn main() {
    let proc = gen_test_code();

    let callable = ScmCallable::CustomProc(proc);
    let res = exec_callable(callable, &Vec::new());
    println!("{}", res);
}
