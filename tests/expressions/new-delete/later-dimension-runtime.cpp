// tidal-rules: NEW-11
// tidal-expect: reject
// tidal-error-clang: array size is not a constant expression
// tidal-error-gcc: array size in new-expression must be constant
// tidal-error-edg: must have a constant value
// tidal-note: Gap review 2026-10-31.
void f(int n, int m) { auto p = new int[n][m]; }
