// tidal-rules: DTOR-3
// tidal-expect: accept
struct S { ~S() = delete; };
S* make() { return new S; }
