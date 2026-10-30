// tidal-rules: CX-3
// tidal-expect: accept
constexpr int f(bool b) {
  if (b) return 1;
  goto end;
  end:
  return 2;
}
static_assert(f(true) == 1);
