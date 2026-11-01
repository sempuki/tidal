// tidal-rules: PFX-20
// tidal-expect: reject
// tidal-error-clang: reference to non-static member function must be called
// tidal-error-gcc: cannot convert .C::f.|invalid use of non-static member function
// tidal-error-edg: bound function may only be used to call
// tidal-note: x.f names a non-static member function, and the expression is not called.
struct C { int f(int); } x;
auto g = x.f;
