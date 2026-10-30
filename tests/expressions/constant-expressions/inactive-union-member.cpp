// tidal-rules: CE-6
// tidal-expect: reject
union U { int i; float f; };
constexpr int f() {
  U u{.f = 1.0f};
  return u.i;
}
static_assert(f() == 0);
