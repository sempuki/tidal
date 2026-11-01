// tidal-rules: PFX-20
// tidal-expect: reject
// tidal-error-clang: reference to non-static member function must be called
// tidal-error-gcc: invalid use of non-static member function
// tidal-error-edg: bound function may only be used to call
// tidal-note: An expression statement that names a member function through an object does not call it.
struct C { int f(int); } x;
void use() { x.f; }
