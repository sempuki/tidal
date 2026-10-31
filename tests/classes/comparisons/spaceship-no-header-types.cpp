// tidal-rules: DCMP-4
// tidal-expect: reject
struct S { int x; auto operator<=>(const S&) const = default; };
bool f(S a, S b) { return a < b; }
