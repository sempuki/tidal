// tidal-rules: LOG-3
// tidal-expect: reject
// tidal-error-clang: invalid argument type .E. to unary expression
// tidal-error-gcc: no match for .operator!.
// tidal-error-edg: must have bool type
enum class E { A };
bool b = !E::A;
