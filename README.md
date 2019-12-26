<h1>Rispy version 0.1.0<h1>

<h4>Directions to compile...so far<h4>

cc -std=c99 -c -ledit -lm mpc.c

//ar rc libmpc.a mpc.o
gcc -g -fPIC mpc.c -shared -o libmpc.so

cp libmpc.so interpreter/target/deps

cargo build
