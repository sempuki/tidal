// tidal-rules: OPR-10
// tidal-expect: reject
using Size = decltype(sizeof 0);
Size operator""_n(const char*, Size n) { return n; }
Size operator""_m(const char*, Size n) { return n; }
auto x = "ab"_n "cd"_m;
