// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int x; friend auto operator==(const S&, const S&) = default; };
bool b = S{1} == S{1};
