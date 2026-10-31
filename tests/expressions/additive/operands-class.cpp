// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*S.*int
// tidal-error-gcc: no match for .operator-.
// tidal-error-edg: no operator .-. matches these operands
struct S {};
auto r = S{} - 1;
