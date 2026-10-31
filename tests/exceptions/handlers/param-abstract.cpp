// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: type .A. is an abstract class
// tidal-error-gcc: catch. parameter to be of abstract class type
// tidal-error-edg: abstract class type .A. is not allowed as catch type
struct A { virtual void f() = 0; };
void g() { try { } catch (A) { } }
