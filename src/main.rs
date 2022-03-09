use project::*;

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

    instr.push(ScmProcUnit::Assign(String::from("myvar")));
    instr.push(ScmProcUnit::Val(ScmValue::Integer(666)));

    return ScmProcedure {
        params: Vec::<String>::new(),
        instructions: instr,
    };
}

#[allow(unused)]
fn print_proc(proc: &ScmProcedure) {
    for instr in &proc.instructions {
        println!("{:?}", instr);
    }
}

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // if args.len() != 2 {
    //     panic!("Wrong arguments number");
    // }

    // let filename = &args[1];
    // let code = std::fs::read_to_string(filename).expect("Error with file reading");

    let mut ctx = ScmExecContext::new();
    // let mut parser = Parser::new(&code);

    // let mut parser = Parser::new("(display (+ (- 3 2) 2)) (newline) (display 666)");
    let mut parser = Parser::new("(if #f (display 142) (display 666))");
    // let callable = ScmCallable::CustomProc(gen_test_code3());
    let callables = parser.parse();

    // if let ScmCallable::CustomProc(proc) = &callables[0] {
    //     print_proc(proc);
    // }

    for callable in callables.iter() {
        exec_callable(&mut ctx, callable.clone(), &Vec::new());
    }
    println!();
}
