// tidal-rules: CVF-3
// tidal-expect: reject
struct H { explicit operator bool() const { return true; } };
bool f(H a) { return a == true; }
