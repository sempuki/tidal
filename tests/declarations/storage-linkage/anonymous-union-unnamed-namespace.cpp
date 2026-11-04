// tidal-rules: STO-11
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: namespace-scope anonymous aggregates must be static
// tidal-error-edg: must be declared static
// tidal-note: Q-0045. A name in an unnamed namespace already has internal linkage, so the anonymous union needs no static. Clang accepts. GCC and EDG require static.
namespace { union { int c; float d; }; }
int f() { return c; }
