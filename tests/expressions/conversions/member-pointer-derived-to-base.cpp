// tidal-rules: CONV-17
// tidal-expect: reject
// tidal-error-clang: different classes
// tidal-error-gcc: cannot convert .int D::\*. to .int B::\*.
// tidal-error-edg: int D::\*. cannot be used to initialize an entity of type .int B::\*
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct D : B { int d; };
int B::* pm = &D::d;
