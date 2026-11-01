// tidal-rules: CPT-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: EDG rejects a trailing clause on a lambda that is not generic, inside a function template (Q-0260).
template <class T> constexpr int run() {
  auto lam = [](int x) requires (sizeof(T) > 1) { return x; };
  return lam(1);
}
static_assert(run<int>() == 1);
