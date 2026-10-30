// tidal-rules: CE-6
// tidal-expect: accept
union U { int i; float f; };
constexpr int f() {
  U u{.i = 4};
  u.f = 2.0f;
  u.i = 9;
  return u.i;
}
static_assert(f() == 9);
