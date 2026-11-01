// tidal-rules: LIST-6
// tidal-expect: accept
constexpr signed char low[] = "ab";
constexpr unsigned char high[] = {"ab"};
constexpr unsigned char wide[4]{"ab"};
static_assert(sizeof(low) == 3 && low[1] == 'b' && low[2] == 0);
static_assert(sizeof(high) == 3 && high[0] == 'a');
static_assert(wide[2] == 0 && wide[3] == 0);
