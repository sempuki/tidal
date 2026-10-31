// tidal-rules: NEW-7
// tidal-expect: reject
// tidal-error-clang: cannot allocate reference type
// tidal-error-gcc: new cannot be applied to a reference type
// tidal-error-edg: type must be an object type
void f() { auto p = new int&; }
