
#[derive(Debug)]
enum Literal{
    Literal(String),
    ComplementedLiteral(String)
}

#[derive(Debug)]
struct Clause{
    literals: Vec<Literal>
}

impl Clause {
    fn new() -> Self {
        Self {
            literals: vec![]
        }
    }

    fn add(&mut self, literal: Literal) {
       self.literals.push(literal); 
    }
}

  
#[derive(Debug)]
struct CNF {
    clauses: Vec<Clause>
}

impl CNF {
    fn new() -> Self {
        Self {
            clauses: vec![]
        }
    }
    fn add(&mut self, clause: Clause) {
       self.clauses.push(clause); 
    }
}



fn parse_cnf(cnf: &str) -> Option<CNF> {
    let modified_cnf = cnf.to_lowercase().replace(" ", "");
    let clauses: Vec<&str> = modified_cnf.split("&&").collect();
    if clauses.is_empty() {
        None
    }else{
        let clauses: Vec<Vec<&str>> = clauses.iter().map(|&clause| {
            clause.trim_start_matches("(")
                  .trim_end_matches(")")
                  .split("||")
                  .collect()
        }).collect();

        let mut cnf = CNF::new();
        for clause in clauses.iter(){
            let mut c = Clause::new();
            for literal in clause.iter() {
                if literal.chars().count() == 1 {
                    c.add(Literal::Literal(literal.to_string()));
                } else {
                    c.add(Literal::ComplementedLiteral(literal.to_string()));
                }
            }
            cnf.add(c);
        }
        Some(cnf)
    }
}


fn main() {
    println!("Enter a CNF: ");
    let cnf = "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)";
    let conjuctions = parse_cnf(cnf);
    dbg!(conjuctions);
}

