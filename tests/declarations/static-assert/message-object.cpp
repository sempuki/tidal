// tidal-rules: SA-8
// tidal-expect: reject
// tidal-error-clang: string literal|user-generated message
// tidal-error-gcc: non-string message
// tidal-error-edg: expected a string literal
struct Msg {
  constexpr unsigned long size() const { return 2; }
  constexpr const char* data() const { return "ok"; }
};
static_assert(true, Msg{});
