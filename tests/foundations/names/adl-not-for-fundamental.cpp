// tidal-rules: NAM-5
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .twice
// tidal-error-gcc: twice. was not declared in this scope
// tidal-error-edg: identifier .twice. is undefined
namespace n { int twice(int v) { return v * 2; } }
int main() { return twice(3); }
