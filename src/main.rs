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

    fn reduce(&mut self, literal:&Literal) {
        match literal {
            Literal::Literal(value) => {
                if self.literals.contains(literal) {
                    self.set_state(State::SATISFIABLE);       
                } else if self.literals.contains(&Literal::ComplementedLiteral(value.to_owned())) {
                    self.literals.remove(&Literal::ComplementedLiteral(value.to_owned()));
                    if self.is_empty() {
                        self.set_state(State::UNSATISFIABLE);
                    }
                }
            },
            Literal::ComplementedLiteral(value) => {
                if self.literals.contains(literal) {
                    self.literals.remove(&Literal::Literal(value.to_owned()));
                    if self.is_empty() {
                        self.set_state(State::UNSATISFIABLE);
                    }
                } else if self.literals.contains(&Literal::Literal(value.to_owned())) {
                    self.set_state(State::SATISFIABLE);       
                }
            }
        }
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

    fn reduce(&mut self, literal: Literal){
        let mut state = State::SATISFIABLE;
        for clause in self.clauses.iter_mut() {
            if clause.is_statifiable() {
                continue;
            }
            clause.reduce(&literal);
            if clause.is_unstatifiable() {
                state = State::UNSATISFIABLE;
                break;
            } else if clause.is_undertimed() {
                state = State::UNDERTIMINED;
            }
        }
        self.set_state(state);
    }
}


fn parse_cnf(cnf: &str) -> Option<CNF> {
    let modified_cnf = cnf.to_lowercase().replace(" ", "");
    let clauses: Vec<&str> = modified_cnf.split("&&").collect();
    if clauses.is_empty() {
        None
    } else {
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

        let mut cnf = CNF::new();
        for clause in clauses.iter() {
            let mut c = Clause::new();
            for literal in clause.iter() {
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
        Some(cnf)
    }
}

fn main() {
    println!("==== SAT SOLVER ====\n");
    println!("Input => CNF expression");
    println!("OR => ||");
    println!("AND => &&");
    println!("Complement => '");
    println!("Literals => any character");
    println!("Group literal => ()");
    print!("\n\n");
    println!("Enter a CNF: ");
    println!("(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)");
    let cnf_str = "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)";
    let mut cnf = parse_cnf(cnf_str).unwrap();
    cnf.reduce(Literal::Literal('a'));
    cnf.reduce(Literal::Literal('c'));
    cnf.reduce(Literal::Literal('b'));
    dbg!(cnf);
}
