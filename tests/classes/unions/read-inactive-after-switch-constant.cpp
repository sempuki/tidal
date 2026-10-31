// tidal-rules: UNI-6
// tidal-expect: reject
// tidal-error-gcc: member instead of (active|initialized)
// tidal-error-clang: read of member .* of union with active member
// tidal-error-edg: invalid access to inactive member
union U { int i; float f; };
constexpr int f() { U u{.i = 1}; u.f = 2.0f; return u.i; }
static_assert(f() == 1);
