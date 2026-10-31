// tidal-rules: OPR-12
// tidal-expect: accept
struct S;
S* take(S& s) { return &s; }
struct S { S* operator&() { return nullptr; } };
