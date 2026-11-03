// tidal-rules: NS-13
// tidal-expect: reject
// tidal-error-clang: call to .g. is ambiguous
// tidal-error-gcc: call of overloaded .g\(\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .g. matches
// tidal-note: Two functions with the same parameters, one in the unnamed namespace and one in the enclosing namespace. The call is ambiguous.
namespace { int g() { return 1; } }
int g() { return 2; }
int w = g();
