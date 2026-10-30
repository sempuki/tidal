// tidal-rules: MUL-4
// tidal-expect: accept
enum Plain { Seven = 7 };
static_assert(Seven % 4 == 3);
static_assert(true % 2 == 1);
static_assert('a' % 10 == 7);
