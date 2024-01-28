use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
    Literal(char),
    ComplementedLiteral(char),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum State {
    SATISFIABLE,
    UNSATISFIABLE,
    UNDERTIMINED,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Clause {
    literals: HashSet<Literal>,
    state: State,
}

impl Clause {
    pub fn new() -> Self {
        Self {
            literals: HashSet::new(),
            state: State::UNDERTIMINED,
        }
    }

    pub fn add(&mut self, literal: Literal) {
        self.literals.insert(literal);
    }

    pub fn clear(&mut self) {
        self.literals.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn is_undertimed(&self) -> bool {
        self.state == State::UNDERTIMINED
    }

    pub fn is_unstatifiable(&self) -> bool {
        self.state == State::UNSATISFIABLE
    }

    pub fn is_statifiable(&self) -> bool {
        self.state == State::SATISFIABLE
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn reduce(&self, literal: &Literal) -> Clause {
        let mut reduced_clause = Self {
            literals: self.literals.clone(),
            state: State::UNDERTIMINED,
        };
        match literal {
            Literal::Literal(value) => {
                if reduced_clause.literals.contains(literal) {
                    reduced_clause
                        .literals
                        .remove(&Literal::Literal(value.to_owned()));
                    reduced_clause.set_state(State::SATISFIABLE);
                } else if reduced_clause
                    .literals
                    .contains(&Literal::ComplementedLiteral(value.to_owned()))
                {
                    reduced_clause
                        .literals
                        .remove(&Literal::ComplementedLiteral(value.to_owned()));
                    if reduced_clause.is_empty() {
                        reduced_clause.set_state(State::UNSATISFIABLE);
                    }
                }
            }
            Literal::ComplementedLiteral(value) => {
                if reduced_clause.literals.contains(literal) {
                    reduced_clause
                        .literals
                        .remove(&Literal::Literal(value.to_owned()));
                    if reduced_clause.is_empty() {
                        reduced_clause.set_state(State::UNSATISFIABLE);
                    }
                } else if reduced_clause
                    .literals
                    .contains(&Literal::Literal(value.to_owned()))
                {
                    reduced_clause
                        .literals
                        .remove(&Literal::ComplementedLiteral(value.to_owned()));
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
pub struct CNF {
    clauses: Vec<Clause>,
    state: State,
}

impl CNF {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
            state: State::UNDERTIMINED,
        }
    }
    pub fn add(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    pub fn is_statifiable(&self) -> bool {
        self.state == State::SATISFIABLE
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn reduce(&mut self, literal: Literal) -> CNF {
        let mut state = State::SATISFIABLE;
        let mut reduced_cnf = Self {
            clauses: Vec::new(),
            state: State::UNDERTIMINED,
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

pub struct CNFWrapper {
    pub cnf: CNF,
    pub literals: HashSet<char>,
}

impl CNFWrapper {
    pub fn new() -> Self {
        Self {
            cnf: CNF::new(),
            literals: HashSet::new(),
        }
    }
}
