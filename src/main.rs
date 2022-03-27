use project::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut filename = &"test.scm".to_owned();

    if args.len() == 2 {
        filename = &args[1];
    }

    let code = std::fs::read_to_string(filename).expect("Error with file reading");

    let mut ctx = ScmExecContext::new();
    let mut parser = Parser::new(&code);

    let callables = parser.parse();

    for callable in callables.into_iter() {
        // if let ScmCallable::CustomProc(proc) = &callable {
        //     println!("{}", proc);
        // }
        exec_callable(&mut ctx, &callable, &Vec::new());
    }
    println!();
}
