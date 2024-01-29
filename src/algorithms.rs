use std::collections::HashSet;

use crate::core::{Literal, State, CNF};


pub fn dpll(cnf: &mut CNF, literals:&mut HashSet<char>) -> State {

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
