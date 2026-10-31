// tidal-rules: NAM-4
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .a
// tidal-error-gcc: a. was not declared in this scope
// tidal-error-edg: identifier .a. is undefined
namespace n { int a = 1; }
int main() { return a; }
