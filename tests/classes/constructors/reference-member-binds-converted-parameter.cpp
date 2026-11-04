// tidal-rules: CTOR-17
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: binds to a temporary object whose lifetime would be shorter
// tidal-note: Q-0084. The parameter has type long, so binding the const int& member creates a temporary int that ends with the constructor. The program never reads the member. Clang rejects. GCC and EDG accept.
struct S {
  const int& r;
  S(long x) : r(x) {}
};
S s(5);
