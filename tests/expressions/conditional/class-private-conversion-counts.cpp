// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-error-clang: conditional expression is ambiguous
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: ambiguous
// tidal-note: Access takes no part in the test of whether a conversion can be formed. Tag converts to Wrap through a public constructor, and Wrap converts to Tag through a private conversion function. Both conversions count, so the expression is ambiguous.
struct Tag;
struct Wrap { Wrap(); Wrap(const Tag&); private: operator Tag() const; };
struct Tag { };
Wrap w;
Tag t;
extern bool c;
auto r = c ? w : t;
