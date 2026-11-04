// tidal-rules: CE-17
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-clang-18: reject
// tidal-error-clang-18: never produces a constant expression
// tidal-error-gcc: non-constant condition for static assertion
// tidal-note: Q-0335. A constexpr function makes a temporary of a class that is not a literal type. Clang 22 and EDG accept. GCC and Clang 18 reject.
struct Port {
  volatile int level;
  int code;
  constexpr Port(int c) : level(0), code(c) {}
  constexpr int get() const { return code; }
};
constexpr int read() { return Port(3).get(); }
static_assert(read() == 3);
