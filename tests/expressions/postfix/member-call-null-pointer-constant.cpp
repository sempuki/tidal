// tidal-rules: PFX-21 CE-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: dereferencing a null pointer|non-constant condition
// tidal-error-clang: member call on null pointer|not an integral constant expression
// tidal-note: Q-0318. The member function reads no member. Clang and GCC reject the call through a null pointer in constant evaluation. EDG accepts it.
struct Gauge { constexpr int one() const { return 1; } };
constexpr int call() {
  const Gauge* p = nullptr;
  return p->one();
}
static_assert(call() == 1);
