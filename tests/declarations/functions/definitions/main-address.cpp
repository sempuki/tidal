// tidal-rules: FN-4
// tidal-expect: reject
// tidal-error-clang: main. to be used by a program|referring to .main. within an expression
// tidal-error-gcc: taking address of function .::main
// tidal-error-edg: main. may not be called or have its address taken
int main() { return 0; }
int (*p)() = &main;
