// tidal-rules: TPL-10
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: lvalue required as left operand of assignment
// tidal-error-edg: expression must be a modifiable lvalue
template <int N> int bump() { N = N + 1; return N; }
int use() { return bump<1>(); }
