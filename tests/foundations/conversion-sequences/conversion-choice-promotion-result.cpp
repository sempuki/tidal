// tidal-rules: ICS-37
// tidal-expect: accept
// tidal-note: The two conversion functions tie on their object parameters. float to double is a promotion and int to double is a conversion, so the function that gives float wins.
struct Source {
  constexpr operator float() const { return 1; }
  constexpr operator int() const { return 2; }
};
constexpr double d = Source{};
static_assert(d == 1);
