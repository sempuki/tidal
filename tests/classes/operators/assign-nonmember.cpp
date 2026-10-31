// tidal-rules: OPR-2
// tidal-expect: reject
struct S {};
S& operator=(S&, int);
