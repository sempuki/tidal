// tidal-rules: RET-6
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-note: Gap review 2026-10-30.
constexpr decltype(auto) local_ref() {
  int x = 1;
  return (x);
}
static_assert(local_ref() == 1);
