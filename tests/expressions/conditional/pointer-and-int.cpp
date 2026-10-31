// tidal-rules: COND-5
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
void f(bool c, int* p) { auto r = c ? p : 1; }
