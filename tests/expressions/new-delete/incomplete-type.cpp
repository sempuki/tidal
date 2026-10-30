// tidal-rules: NEW-7
// tidal-expect: reject
struct Incomplete;
void f() { auto p = new Incomplete; }
