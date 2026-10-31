// tidal-rules: UN-2
// tidal-expect: reject
// tidal-error-clang: invalid argument type .E. to unary expression
// tidal-error-gcc: no match for .operator-.
// tidal-error-edg: no operator .-. matches these operands
enum class E { A };
auto x = -E::A;
