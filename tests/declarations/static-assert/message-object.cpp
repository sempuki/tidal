// tidal-rules: SA-4
// tidal-expect: reject
struct Msg {
  constexpr unsigned long size() const { return 2; }
  constexpr const char* data() const { return "ok"; }
};
static_assert(true, Msg{});
