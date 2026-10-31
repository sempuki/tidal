// tidal-rules: CPY-16
// tidal-expect: reject
struct S {};
S& operator=(S& a, const S&) { return a; }
