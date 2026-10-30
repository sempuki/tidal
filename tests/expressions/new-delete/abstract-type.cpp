// tidal-rules: NEW-7
// tidal-expect: reject
struct Shape { virtual int area() = 0; };
void f() { auto p = new Shape; }
