
use sat_solver::parser::parse_cnf;
use sat_solver::core::State;
use sat_solver::algorithms::dpll;


 

fn main() {
    println!("==== SAT SOLVER ====\n");
    println!("Input => CNF expression");
    println!("OR => ||");
    println!("AND => &&");
    println!("Complement => '");
    println!("Literals => any character");
    println!("Group literal => ()");
    println!("Example CNF: (a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)");
    print!("\n\n");
    println!("Enter a CNF: ");
    let cnf_str = "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)";
    let cnf_str = "(a)&&(a')";
    let mut cnf_wrapper = parse_cnf(cnf_str).unwrap();
    let result = dpll(&mut cnf_wrapper.cnf, &mut cnf_wrapper.literals); 
    if result == State::SATISFIABLE {
        println!("SATISFIABLE");
    } else {
        println!("UN-SATISFIABLE");
    }
}
