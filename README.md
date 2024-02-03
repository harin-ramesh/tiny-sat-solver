### tiny-sat-solver

This is a simple sat solver using dpll algorithm, this is a very simple implementation.

---
#### How to use
###### Clone the the repo
```shell
git clone https://github.com/harin-ramesh/tiny-sat-solver
```
##### Build
```shell
cargo build --release
```

##### Use the below command to check satifiability
```
# Help
./target/release/sat-solver -h

# To check Satisfiability
./target/release/sat-solver -c "(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)"
./target/release/sat-solver -c "(a)&&(a')"
```
