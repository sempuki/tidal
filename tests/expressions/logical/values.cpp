// tidal-rules: LOG-10
// tidal-expect: accept
static_assert(true && true);
static_assert(!(true && false));
static_assert(false || true);
static_assert(!(false || false));
static_assert(!false && !!true);
