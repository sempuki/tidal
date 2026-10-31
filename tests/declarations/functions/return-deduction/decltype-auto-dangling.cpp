// tidal-rules: RET-6
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: read of variable whose lifetime has ended
// tidal-error-edg: attempt to access expired storage
// tidal-note: Gap review 2026-10-30.
constexpr decltype(auto) local_ref() {
  int x = 1;
  return (x);
}
static_assert(local_ref() == 1);
