// tidal-rules: LOG-6
// tidal-expect: accept
static_assert(true || false && false);
static_assert(!((true || false) && false));
static_assert(!false == true);
