// tidal-rules: DTOR-3
// tidal-expect: accept
// tidal-note: The braces initialize the member v of the new object. No prvalue of S occurs, so the deleted destructor is not needed.
struct S { int v; ~S() = delete; };
S* p = new S{1};
