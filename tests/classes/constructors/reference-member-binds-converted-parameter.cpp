// tidal-rules: CTOR-17
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: binds to a temporary object whose lifetime would be shorter
// tidal-note: Q-0084. Clang rejects, GCC and EDG accept. The parameter has type long, so binding the const int& member creates a temporary int that ends with the constructor. The program never reads the member.
struct S {
  const int& r;
  S(long x) : r(x) {}
};
S s(5);
