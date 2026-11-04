// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-error-clang: conditional expression is ambiguous
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: ambiguous
// tidal-note: Meter converts to int, and int converts to Meter through the constructor. Both conversions succeed, so the expression is not valid.
struct Meter { Meter(int); operator int() const; };
Meter m(0);
extern bool c;
auto r = c ? m : 1;
