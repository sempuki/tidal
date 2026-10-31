// tidal-rules: ALN-3
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: must be specified on definition
// tidal-error-edg: must also appear on definition
// tidal-note: Q-0070. GCC accepts declarations of one variable with different alignas. Clang and EDG reject them.
alignas(16) extern int x;
int x;
