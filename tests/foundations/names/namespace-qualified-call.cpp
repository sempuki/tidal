// tidal-rules: NAM-9
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .twice
// tidal-error-gcc: twice. was not declared in this scope
// tidal-error-edg: identifier .twice. is undefined
// tidal-note: An unqualified call of a function of the namespace. The argument is an int, so no namespace is associated with the call.
namespace n { int twice(int v) { return v * 2; } }
int main() { return twice(2); }
