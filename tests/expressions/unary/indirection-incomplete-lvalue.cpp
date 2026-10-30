// tidal-rules: UN-5
// tidal-expect: accept
struct Incomplete;
Incomplete& get(Incomplete* p) { return *p; }
