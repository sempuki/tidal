// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: ambiguous
// tidal-note: Q-0378. A deleted conversion function still counts in the test of whether a conversion can be formed, so both conversions count and the expression is ambiguous. GCC and EDG reject. Clang leaves out the deleted conversion and accepts.
struct Tag;
struct Wrap { Wrap(); Wrap(const Tag&); operator Tag() const = delete; };
struct Tag { };
Wrap w;
Tag t;
extern bool c;
auto r = c ? w : t;
