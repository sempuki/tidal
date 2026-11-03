// tidal-rules: TYP-11
// tidal-expect: accept
// tidal-note: Conversions from a pointer and from a double, and false and true in a subtraction.
constexpr int x = 0;
static_assert(bool(&x) == true);
static_assert(bool(0.5) == true && bool(0.0) == false);
static_assert(false - true == -1);
constexpr bool none(nullptr);
static_assert(!none);
