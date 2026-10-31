// tidal-rules: CMP-8
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31. Constant evaluation gives a result for pointers to two members of a union, although the order is unspecified.
union U { int a; int b; };
constexpr U u{};
constexpr bool lt = static_cast<const void*>(&u.a) < static_cast<const void*>(&u.b);
constexpr bool gt = static_cast<const void*>(&u.a) > static_cast<const void*>(&u.b);
