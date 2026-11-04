// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: redefinition of .S. as different kind of symbol
// tidal-note: Q-0362. A member of an anonymous union is a data member of C, and a class in C can share its name. GCC and EDG accept. Clang rejects.
struct C { union { int S; }; struct S {}; };
