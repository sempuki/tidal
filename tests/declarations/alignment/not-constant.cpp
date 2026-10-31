// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-error-clang: requires integer constant
// tidal-error-gcc: not an integer constant|not usable in a constant expression
// tidal-error-edg: must have a constant value
int n = 8;
alignas(n) int x;
