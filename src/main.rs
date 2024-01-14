
fn parse_cnf(cnf: &str) -> Option<Vec<Vec<&str>>> {
    let clauses: Vec<&str> = cnf.split("&&").collect();
    if clauses.is_empty() {
        None
    }else{
        let clauses: Vec<Vec<&str>> = clauses.iter().map(|&clause| {
            clause.trim_start_matches("(")
                  .trim_end_matches(")")
                  .split("||")
                  .collect()
        }).collect();
        Some(clauses)
    }
}


fn main() {
    println!("Enter a CNF: ");
    let cnf = "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)".to_lowercase().replace(" ", "");
    let conjuctions = parse_cnf(&cnf);
    dbg!(conjuctions);
}

