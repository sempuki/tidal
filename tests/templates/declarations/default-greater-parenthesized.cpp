// tidal-rules: TPL-39 TPL-13
// tidal-expect: accept
template <bool B = (3 > 4)> struct Flag { static constexpr bool value = B; };
static_assert(!Flag<>::value);
