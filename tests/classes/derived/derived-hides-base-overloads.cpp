// tidal-rules: DER-19
// tidal-expect: reject
// tidal-error-clang: cannot initialize a parameter of type .const char \*. with an rvalue of type .int
// tidal-error-gcc: invalid conversion from .int. to .const char\*
// tidal-error-edg: argument of type .int. is incompatible with parameter of type .const char \*
struct B { void f(int); };
struct D : B { void f(const char*); };
void g(D& d) { d.f(1); }
