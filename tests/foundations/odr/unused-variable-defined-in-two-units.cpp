// tidal-rules: ODR-8
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of .counter
// tidal-error-gcc: multiple definition of .counter
// tidal-sources: unused-variable-defined-in-two-units.tu.cpp
// tidal-note: Two units define counter, which the program never uses.
int counter = 1;
int main() { return 0; }
