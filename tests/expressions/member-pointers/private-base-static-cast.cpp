// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: its private base class
// tidal-error-gcc: is an inaccessible base of
// tidal-error-edg: inaccessible base class
// tidal-note: Gap review 2026-10-31.
struct B { int x; };
struct D : private B { int y; };
int B::* pm = static_cast<int B::*>(&D::y);
