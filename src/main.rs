use std::hash::{Hash, Hasher};
use std::collections::HashSet;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Literal {
    Literal(char),
    ComplementedLiteral(char),
}


#[derive(Debug, Hash, PartialEq, Eq)]
enum State {
    SATISFIABLE,
    UNSATISFIABLE,
    UNDERTIMINED
}

#[derive(Debug, PartialEq, Eq)]
struct Clause {
    literals: HashSet<Literal>,
    state: State
}

impl Clause {
    fn new() -> Self {
        Self { literals: HashSet::new(), state:State::UNDERTIMINED }
    }

    fn add(&mut self, literal: Literal) {
        self.literals.insert(literal);
    }

    fn clear(&mut self) {
        self.literals.clear();
    }

    fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    fn is_undertimed(&self) -> bool{
        self.state == State::UNDERTIMINED
    }

    fn is_unstatifiable(&self) -> bool{
        self.state == State::UNSATISFIABLE
    }

    fn is_statifiable(&self) -> bool{
        self.state == State::SATISFIABLE
    }

    fn set_state(&mut self, state: State){
        self.state = state;
    }

    fn reduce(&self, literal:&Literal) -> Clause {
        let mut reduced_clause = Self { literals: self.literals.clone(), state:State::UNDERTIMINED };
        match literal {
            Literal::Literal(value) => {
                if reduced_clause.literals.contains(literal) {
                    reduced_clause.literals.remove(&Literal::Literal(value.to_owned()));
                    reduced_clause.set_state(State::SATISFIABLE);       
                } else if reduced_clause.literals.contains(&Literal::ComplementedLiteral(value.to_owned())) {
                    reduced_clause.literals.remove(&Literal::ComplementedLiteral(value.to_owned()));
                    if reduced_clause.is_empty() {
                        reduced_clause.set_state(State::UNSATISFIABLE);
                    }
                }
            },
            Literal::ComplementedLiteral(value) => {
                if reduced_clause.literals.contains(literal) {
                    reduced_clause.literals.remove(&Literal::Literal(value.to_owned()));
                    if reduced_clause.is_empty() {
                        reduced_clause.set_state(State::UNSATISFIABLE);
                    }
                } else if reduced_clause.literals.contains(&Literal::Literal(value.to_owned())) {
                    reduced_clause.literals.remove(&Literal::ComplementedLiteral(value.to_owned()));
                    reduced_clause.set_state(State::SATISFIABLE);       
                }
            }
        }
        reduced_clause
    }
}


impl Hash for Clause {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let literals: Vec<_> = self.literals.iter().collect();
        literals.hash(state);
        self.state.hash(state);
    }
}


#[derive(Debug)]
struct CNF {
    clauses: Vec<Clause>,
    state: State
}

impl CNF {
    fn new() -> Self {
        Self { clauses: Vec::new(), state: State::UNDERTIMINED }
    }
    fn add(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
    fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    fn is_statifiable(&self) -> bool{
        self.state == State::SATISFIABLE
    }

    fn set_state(&mut self, state: State){
        self.state = state;
    }

    fn reduce(&mut self, literal: Literal) -> CNF {
        let mut state = State::SATISFIABLE;
        let mut reduced_cnf = Self { 
            clauses: Vec::new(),
            state: State::UNDERTIMINED
        };
        for clause in self.clauses.iter_mut() {
            if clause.is_statifiable() {
                continue;
            }
            let reduced_clause = clause.reduce(&literal);
            if reduced_clause.is_unstatifiable() {
                state = State::UNSATISFIABLE;
                break;
            } else if reduced_clause.is_undertimed() {
                state = State::UNDERTIMINED;
            }
            reduced_cnf.clauses.push(reduced_clause);
        }
        reduced_cnf.set_state(state);
        reduced_cnf
    }
}


struct CNFWrapper {
    cnf: CNF,
    literals: HashSet<char>
}


impl CNFWrapper {
    fn new() -> Self {
        Self {
            cnf: CNF::new(),
            literals: HashSet::new()
        }
    }
}


fn parse_cnf(cnf: &str) -> Option<CNFWrapper> {
    let modified_cnf = cnf.to_lowercase().replace(" ", "");
    let clauses: Vec<&str> = modified_cnf.split("&&").collect();
    if clauses.is_empty() {
        None
    } else {
        let mut cnf_wrapper = CNFWrapper::new();
        let clauses: Vec<Vec<&str>> = clauses
            .iter()
            .map(|&clause| {
                clause
                    .trim_start_matches("(")
                    .trim_end_matches(")")
                    .split("||")
                    .collect()
            })
            .collect();

        let cnf = &mut cnf_wrapper.cnf;
        for clause in clauses.iter() {
            let mut c = Clause::new();
            for literal in clause.iter() {
                cnf_wrapper.literals.insert(literal.chars().nth(0).unwrap().to_owned());
                if literal.chars().count() == 1 {
                    c.add(Literal::Literal(literal.chars().nth(0).unwrap().to_owned()));
                } else {
                    c.add(Literal::ComplementedLiteral(
                        literal.chars().nth(0).unwrap().to_owned(),
                    ));
                }
            }
            cnf.add(c);
        }
        Some(cnf_wrapper)
    }
}


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
    let mut cnf_wrapper = parse_cnf(cnf_str).unwrap();
    dbg!(dpll(&mut cnf_wrapper.cnf, &mut cnf_wrapper.literals));
    

/*
    let mut x = cnf_wrapper.cnf.reduce(Literal::Literal('a'));
    println!("{:?}", x);
    let mut x = x.reduce(Literal::Literal('c'));
    println!("{:?}", x);
    let mut x = x.reduce(Literal::Literal('b'));
    println!("{:?}", x);
*/
}
