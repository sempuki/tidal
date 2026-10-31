// tidal-rules: STO-11
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: namespace-scope anonymous aggregates must be static
// tidal-error-edg: must be declared static
// tidal-note: Gap review 2026-10-30. Q-0045. GCC and EDG require static on an anonymous union in an unnamed namespace; Clang accepts it.
namespace { union { int c; float d; }; }
int f() { return c; }
