// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: cannot cast private base class
// tidal-error-gcc: is an inaccessible base of
// tidal-error-edg: inaccessible base class
// tidal-note: Gap review 2026-10-31.
struct B { int x; };
struct D : private B {};
int D::* pm = &B::x;
