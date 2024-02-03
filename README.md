### tiny-sat-solver

Input => CNF expression
OR => ||
AND => &&
Complement => '
Literals => any character
Group literal => ()

Current this sat solver uses dpll as algorithm.

```
CNF example:
(a || b || c) && (a’ || b’|| c) && (a’|| b || c’) && (a || b’ || c’)
(a)&&(a')
```


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
