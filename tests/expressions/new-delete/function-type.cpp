// tidal-rules: NEW-7
// tidal-expect: reject
// tidal-error-clang: cannot allocate function type
// tidal-error-gcc: new cannot be applied to a function type
// tidal-error-edg: type must be an object type
using F = int(int);
void f() { auto p = new F; }
