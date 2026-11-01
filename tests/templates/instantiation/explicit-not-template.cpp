// tidal-rules: INST-22
// tidal-expect: reject
// tidal-error-clang: does not refer to a function template
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: declaration is incompatible with "int get\(int\)"
int get(int) { return 1; }
template int get(int);
