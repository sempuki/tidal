// tidal-rules: ENUM-11
// tidal-expect: reject
// tidal-error-clang: invalid argument type .S. to unary expression
// tidal-error-gcc: no match for .operator!
// tidal-error-edg: must have bool type
// tidal-note: Gap review 2026-10-31.
enum class S { off, on };
bool f() { return !S::on; }
