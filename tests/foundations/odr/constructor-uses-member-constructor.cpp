// tidal-rules: ODR-4 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Part::Part\(\)
// tidal-error-gcc: undefined reference to .Part::Part\(\)
struct Part { Part(); };
struct Whole { Part part; };
int main() { Whole whole; (void)whole; return 0; }
