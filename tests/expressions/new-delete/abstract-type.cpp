// tidal-rules: NEW-7
// tidal-expect: reject
// tidal-error-clang: object of abstract class type
// tidal-error-gcc: new-expression of abstract class type
// tidal-error-edg: abstract class type .Shape. is not allowed
struct Shape { virtual int area() = 0; };
void f() { auto p = new Shape; }
