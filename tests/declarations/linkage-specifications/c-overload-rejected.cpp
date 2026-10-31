// tidal-rules: LSP-4
// tidal-expect: reject
// tidal-error-clang: conflicting types for
// tidal-error-gcc: conflicting declaration of C function
// tidal-error-edg: has .C. linkage
extern "C" void f(int);
extern "C" void f(double);
