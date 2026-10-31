// tidal-rules: CE-6
// tidal-expect: reject
// tidal-error-gcc: accessing .U::i. member instead of
// tidal-error-clang: read of member .* of union with active member
// tidal-error-edg: invalid access to inactive member
union U { int i; float f; };
constexpr int f() {
  U u{.f = 1.0f};
  return u.i;
}
static_assert(f() == 0);
