// tidal-rules: OPR-15 OVL-9
// tidal-expect: reject
// tidal-error-clang: use of overloaded operator .%. is ambiguous
// tidal-error-gcc: no match for .operator%.
// tidal-error-edg: no operator .%. matches these operands
// tidal-note: % has candidates for integer operands only. The double converts to every integer type equally well, so no candidate is best. GCC and EDG report no match.
struct ToDouble { operator double() const; };
auto r = ToDouble{} % 2;
