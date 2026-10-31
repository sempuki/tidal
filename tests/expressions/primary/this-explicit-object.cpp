// tidal-rules: PRIM-20
// tidal-expect: reject
// tidal-error-clang: invalid use of .this. in a function with an explicit object parameter
// tidal-error-gcc: unavailable for explicit object member functions
// tidal-error-edg: may only be used inside a nonstatic member function
// tidal-note: Gap review 2026-10-31.
struct S { int v; int get(this S& self) { return this->v; } };
