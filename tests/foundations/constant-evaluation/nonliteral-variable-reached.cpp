// tidal-rules: CE-17
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: never produces a constant expression
// tidal-note: Q-0335. The evaluation makes a local variable of a class that is not a literal type and reads only the member that is not volatile. Clang 22, GCC, and EDG accept. Clang 18 rejects.
struct Port {
  volatile int level;
  int code;
  constexpr Port(int c) : level(0), code(c) {}
  constexpr int get() const { return code; }
};
constexpr int read() { Port p(2); return p.get(); }
static_assert(read() == 2);
