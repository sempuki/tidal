// tidal-rules: ODR-8
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of .helper\(\)
// tidal-error-gcc: multiple definition of .helper\(\)
// tidal-sources: unused-function-defined-in-two-units.tu.cpp
// tidal-note: Two units define helper, which the program never uses.
int helper() { return 1; }
int main() { return 0; }
