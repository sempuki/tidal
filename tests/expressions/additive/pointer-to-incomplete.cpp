// tidal-rules: ADD-3
// tidal-expect: reject
struct Incomplete;
Incomplete* f(Incomplete* p) { return p + 1; }
