// tidal-rules: LOG-3
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool.
// tidal-error-gcc: no match for .operator&&.
// tidal-error-edg: no operator .&&. matches these operands
struct S {};
bool b = S{} && true;
