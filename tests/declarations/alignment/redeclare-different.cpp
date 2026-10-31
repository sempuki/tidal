// tidal-rules: ALN-3
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0070. GCC accepts declarations of one variable with different alignas. Clang and EDG reject them.
alignas(8) extern int x;
alignas(16) int x;
