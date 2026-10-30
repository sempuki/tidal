// tidal-rules: MUL-2
// tidal-expect: accept
enum Plain { Two = 2 };
static_assert(Two * 3 == 6);
static_assert(true * 5 == 5);
static_assert('\x02' * 4 == 8);
static_assert(1.5 * 2 == 3.0);
static_assert(3.0f / 2 == 1.5f);
