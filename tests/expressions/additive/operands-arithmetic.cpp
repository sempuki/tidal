// tidal-rules: ADD-2
// tidal-expect: accept
enum Plain { Two = 2 };
static_assert(Two + 1 == 3);
static_assert(true + true == 2);
static_assert('a' - 'a' == 0);
static_assert(1.5 + 1 == 2.5);
