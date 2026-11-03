// tidal-rules: OPR-12
// tidal-expect: accept
// tidal-note: The object is const, and the class declares a const operator& after the point of use. Either operator is allowed, so the test checks only that the program compiles.
struct S;
const S* take(const S& s) { return &s; }
struct S { int v; const S* operator&() const { return nullptr; } };
