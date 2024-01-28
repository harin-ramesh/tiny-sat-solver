use crate::core::{Literal, CNFWrapper, Clause};

pub fn parse_cnf(cnf: &str) -> Option<CNFWrapper> {
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

