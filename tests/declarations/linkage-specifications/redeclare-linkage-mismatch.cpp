// tidal-rules: LSP-7
// tidal-expect: reject
// tidal-error-clang: different language linkage
// tidal-error-gcc: with .C. linkage
// tidal-error-edg: linkage specification is incompatible
int g(int);
extern "C" int g(int);
