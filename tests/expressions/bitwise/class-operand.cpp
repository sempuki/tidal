// tidal-rules: BIT-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*S
// tidal-error-gcc: no match for .operator&.
// tidal-error-edg: no operator .&. matches these operands
// tidal-note: Gap review 2026-10-31 (third).
struct S {};
int f(S s) { return s & 1; }
