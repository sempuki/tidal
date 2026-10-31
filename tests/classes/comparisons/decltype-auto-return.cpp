// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int x; decltype(auto) operator==(const S&) const = default; };
