use std::env;

use sat_solver::parser::parse_cnf;
use sat_solver::core::State;
use sat_solver::algorithms::dpll;


fn help() {
    println!("==== SAT SOLVER HELP ====\n");
    println!("Input => CNF expression");
    println!("OR => ||");
    println!("AND => &&");
    println!("Complement => '");
    println!("Literals => any character");
    println!("Group literal => ()");
    print!("\n");
    println!("CNF example: ");
    println!("(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)");
    println!("(a)&&(a')");
    print!("\n");
    println!("Example commands");
    println!("cargo run -- -c \"(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)\"");
    println!("cargo run -- -c \"(a)&&(a')\"");
}


fn main() {
    let mut args = env::args();
    args.next();

    if let Some(flag) = args.next() {
        if flag == "-c" {
            if let Some(cnf) = args.next() {
                let mut cnf_wrapper = parse_cnf(&cnf).unwrap();
                let result = dpll(&mut cnf_wrapper.cnf, &mut cnf_wrapper.literals); 
                if result == State::SATISFIABLE {
                    println!("SATISFIABLE");
                } else {
                    println!("UN-SATISFIABLE");
                }
            } else {
                print!("sdfsdfsdF");
            }
        } else if flag == "-h" || flag == "-help" {
            help();   
        } else {
            println!("Unexpected argument: {}", flag);
        }
    } else {
        println!("Missing argument");
    }
}

