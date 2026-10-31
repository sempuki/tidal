// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: main. is not allowed to be declared static
// tidal-error-gcc: cannot declare .::main. to be static
// tidal-error-edg: static. is not allowed
static int main() { return 0; }
