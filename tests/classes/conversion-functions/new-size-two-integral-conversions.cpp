// tidal-rules: CVF-8
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: ambiguous default type conversion
// tidal-error-edg: more than one conversion function
struct S { operator int() const { return 1; } operator long() const { return 2; } };
int* f(S s) { return new int[s]; }
