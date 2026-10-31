// tidal-rules: OBJ-10
// tidal-expect: reject
union U { int i; float f; };
constexpr int f() { U u{.i = 1}; const int& r = u.i; u.f = 2.0f; return r; }
static_assert(f() == 1);
