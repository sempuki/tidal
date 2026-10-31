// tidal-rules: UN-14
// tidal-expect: reject
// tidal-error-clang: sizeof. to a function type
// tidal-error-gcc: sizeof. to an expression of function type
// tidal-error-edg: operand of sizeof may not be a function
void f();
auto n = sizeof(f);
