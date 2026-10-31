// tidal-rules: CEV-2
// tidal-expect: reject
// tidal-error-clang: cannot take address of consteval function
// tidal-error-gcc: taking address of an immediate function
// tidal-error-edg: address of a consteval function cannot be used here
consteval int one() { return 1; }
int (*p)() = one;
