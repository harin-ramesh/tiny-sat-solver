#[derive(Debug, Clone)]
enum Literal {
    Literal(char),
    ComplementedLiteral(char),
}

#[derive(Debug)]
struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    fn new() -> Self {
        Self { literals: vec![] }
    }

    fn add(&mut self, literal: Literal) {
        self.literals.push(literal);
    }

    fn clear(&mut self) {
        self.literals.clear();
    }

    fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}

#[derive(Debug)]
struct CNF {
    clauses: Vec<Clause>,
}

impl CNF {
    fn new() -> Self {
        Self { clauses: vec![] }
    }
    fn add(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
}

fn reduce(cnf: CNF, literal: Literal) -> CNF {
    let mut reduced_cnf = CNF::new();
    for clause in cnf.clauses.iter() {
        let mut reduced_clause = Clause::new();
        for lit in clause.literals.iter() {
            match (&literal, &lit) {
                (Literal::Literal(value1), Literal::ComplementedLiteral(value2))
                    if value1 == value2 =>
                {
                    reduced_clause.clear();
                    break;
                }
                (Literal::Literal(value1), Literal::Literal(value2)) if value1 == value2 => {}
                _ => reduced_clause.add(lit.clone()),
            }
        }
        if !reduced_clause.is_empty() {
            reduced_cnf.add(reduced_clause);
        }
    }
    reduced_cnf
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
    let cnf_str = "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)";
    let cnf = parse_cnf(cnf_str);
    let r = reduce(cnf.unwrap(), Literal::Literal('a'));
    dbg!(r);
}
