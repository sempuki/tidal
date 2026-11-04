// tidal-rules: CE-17 CE-23
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error-clang-18: not an integral constant expression
// tidal-error-gcc: non-constant condition for static assertion
// tidal-error-edg: expression must have a constant value
// tidal-note: Q-0335. Port is not a literal type, and the evaluation makes a temporary of it and reads only the member that is not volatile. Only a constexpr variable needs a literal type. Clang 22 accepts. GCC, EDG, and Clang 18 reject.
struct Port {
  volatile int level;
  int code;
  constexpr Port(int c) : level(0), code(c) {}
  constexpr int get() const { return code; }
};
static_assert(Port(1).get() == 1);  // the temporary Port(1) is not of a literal type
