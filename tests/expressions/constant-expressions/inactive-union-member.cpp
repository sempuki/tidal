// tidal-rules: CE-6
// tidal-expect: reject
// tidal-error-gcc: accessing .U::i. member instead of
union U { int i; float f; };
constexpr int f() {
  U u{.f = 1.0f};
  return u.i;
}
static_assert(f() == 0);
