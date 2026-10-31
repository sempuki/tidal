// tidal-rules: TR-4 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .defined_elsewhere
// tidal-error-gcc: defined_elsewhere. was not declared in this scope
// tidal-error-edg: identifier .defined_elsewhere. is undefined
// tidal-sources: unit-sees-own-declarations-only.tu.cpp
int main() { return defined_elsewhere(); }
