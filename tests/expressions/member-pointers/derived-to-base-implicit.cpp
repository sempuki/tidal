// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: different classes
// tidal-error-gcc: cannot convert .int D::\*. to .int B::\*.
// tidal-error-edg: int D::\*. cannot be used to initialize an entity of type .int B::\*
struct B { int x; };
struct D : B { int y; };
int B::* pm = &D::y;
