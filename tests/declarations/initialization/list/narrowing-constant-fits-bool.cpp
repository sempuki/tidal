// tidal-rules: CONV-21 LIST-2
// tidal-expect: accept
// tidal-note: The constants 0 and 1, of type int and of type char, fit in bool, so the conversions do not narrow.
constexpr bool t{1};
constexpr bool f{0};
constexpr char one = 1;
constexpr bool c{one};
static_assert(t && !f && c);
