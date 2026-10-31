// tidal-rules: PFX-12 UN-20
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: value exceeds range of .short
// tidal-note: Q-0124. EDG rejects the step during constant evaluation.
constexpr bool f() {
  short top = 32767;
  short old = top++;
  return old == 32767 && top == -32768;
}
static_assert(f());
