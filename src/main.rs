mod parser;
mod core;

use std::collections::HashSet;

use parser::parse_cnf;
use core::{CNF, Literal, State};


fn dpll(cnf: &mut CNF, literals:&mut HashSet<char>) -> State {

    if literals.is_empty() {
        return State::UNSATISFIABLE;
    }

    let literal = literals.iter().next().cloned().unwrap();
    literals.remove(&literal);

    let mut reduced_cnf = cnf.reduce(Literal::Literal(literal));
    if reduced_cnf.is_statifiable() {
        return State::SATISFIABLE;
    }

    let state = dpll(&mut reduced_cnf, literals);        
    if state == State::SATISFIABLE {
        return State::SATISFIABLE
    }

    let mut reduced_cnf = cnf.reduce(Literal::ComplementedLiteral(literal));
    if reduced_cnf.is_statifiable() {
        return State::SATISFIABLE
    }
    let state = dpll(&mut reduced_cnf, literals);
    literals.insert(literal);
    state
}
 

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
