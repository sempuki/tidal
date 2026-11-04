// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32, where double has alignment 4 (TYP-2). The union rules are the same as on x86-64 (layout-union-size.cpp).
union Five { char c[5]; int i; };
union Wide { char c; double d; };
union Odd { char c[9]; double d; };
union Nothing {};
static_assert(sizeof(Five) == 8 && alignof(Five) == 4);
static_assert(sizeof(Wide) == 8 && alignof(Wide) == 4);
static_assert(sizeof(Odd) == 12);
static_assert(sizeof(Nothing) == 1);
