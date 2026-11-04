// tidal-rules: COND-11 COND-5
// tidal-expect: reject
// tidal-error-clang: conditional expression is ambiguous
// tidal-error-gcc: no match for ternary .operator\?:.
// tidal-error-edg: more than one conversion function from "Wide" to a built-in type
// tidal-note: Wide converts to int and to long, and Narrow to int. The candidates (int, int) and (long, int) use different conversion functions of Wide, so neither is better.
struct Wide { operator int(); operator long(); };
struct Narrow { operator int(); };
extern bool c;
Wide w;
Narrow n;
auto x = c ? w : n;
