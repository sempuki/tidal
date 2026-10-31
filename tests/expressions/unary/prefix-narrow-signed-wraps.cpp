// tidal-rules: UN-20
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: value exceeds range of .signed char
// tidal-note: Q-0124. EDG rejects the step during constant evaluation.
constexpr bool f() {
  signed char low = -128;
  --low;
  return low == 127;
}
static_assert(f());
