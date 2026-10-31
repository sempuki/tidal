// tidal-rules: CVF-1
// tidal-expect: run
// tidal-expect-gcc-host: reject
// tidal-expect-gcc-16: reject
// tidal-error-gcc: invalid use of .auto. in conversion operator
// tidal-note: Q-0059. GCC 16 rejects a conversion function with a deduced return type under -pedantic-errors. GCC 14, Clang, and EDG accept it.
struct S { operator auto() const { return 2.5; } };
int main() { S s; double d = s; return d == 2.5 ? 0 : 1; }
