# Ising

Implementation of the two-dimensional Ising model using the Metropolis-Hastings
algorithm in rust.

## Usage

Use cargo to build a binary:

```shell
cargo build
```

Run the binary and pipe the csv formatted output into a file:

```shell
./target/debug/ising --beta 0.2 --size 32 --steps 50000 > data1.csv
```
