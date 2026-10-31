// tidal-rules: ALN-3
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: must be specified on definition
// tidal-error-edg: must also appear on definition
// tidal-note: Q-0070. GCC accepts a class definition with an alignas different from an earlier declaration. Clang and EDG reject it.
struct alignas(16) S;
struct S { int v; };
