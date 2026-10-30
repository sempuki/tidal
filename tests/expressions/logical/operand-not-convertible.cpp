// tidal-rules: LOG-3
// tidal-expect: reject
struct S {};
bool b = S{} && true;
