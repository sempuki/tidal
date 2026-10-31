// tidal-rules: NS-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .N. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: N. has already been declared in the current scope
int N;
namespace N {}
