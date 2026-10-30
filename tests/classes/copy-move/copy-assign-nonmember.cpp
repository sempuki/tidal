// tidal-rules: CPY-3
// tidal-expect: reject
struct S {};
S& operator=(S& a, const S&) { return a; }
