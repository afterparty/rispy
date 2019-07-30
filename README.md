<h1>Rispy version 0.1.0<h1>

<h4>Directions to compile...so far<h4>

cc -std=c99 -c -ledit -lm mpc.c

ar rc libmpc.a mpc.o

cp libmpc.a interpreter/target/deps

cargo build
