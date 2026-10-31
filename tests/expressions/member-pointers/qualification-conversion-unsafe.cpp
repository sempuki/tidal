// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: type .const int \*S::\*. with an lvalue of type .int \*S::\*.
// tidal-error-gcc: invalid conversion from .int\* S::\*. to .const int\* S::\*.
// tidal-error-edg: cannot be used to initialize an entity of type .const int \*S::\*
// tidal-note: Gap review 2026-10-31.
struct S { int* p; };
int* S::* pp = &S::p;
const int* S::* ppc = pp;
