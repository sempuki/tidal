// tidal-rules: TPL-10
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue of type
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: expression must be an lvalue or a function designator
template <int N> const int* where() { return &N; }
const int* use() { return where<1>(); }
