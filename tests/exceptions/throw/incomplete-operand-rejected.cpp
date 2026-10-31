// tidal-rules: THR-3
// tidal-expect: reject
struct I;
void g(I& i) { throw i; }
