### tiny-sat-solver

Input => CNF expression
OR => ||
AND => &&
Complement => '
Literals => any character
Group literal => ()

CNF example:
(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)
(a)&&(a')

Example commands
cargo run -- -c "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)"
cargo run -- -c "(a)&&(a')"
