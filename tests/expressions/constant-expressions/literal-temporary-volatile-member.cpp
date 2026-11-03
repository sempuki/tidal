// tidal-rules: CE-20 CE-23
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: not an integral constant expression
// tidal-error-gcc: temporary of non-literal type
// tidal-note: Q-0335. GCC, Clang 18, and EDG reject a temporary of a type that is not literal during constant evaluation; Clang 22 accepts it.
struct Port {
  volatile int level;
  int code;
  constexpr Port(int c) : level(0), code(c) {}
  constexpr int get() const { return code; }
};
static_assert(Port(1).get() == 1);  // the temporary Port(1) is not of a literal type
