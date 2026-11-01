// tidal-rules: LOG-6 LOG-13
// tidal-expect: accept
static_assert((not false == true) == true);
static_assert((true or false and false) == true);
static_assert((false and true or true) == true);
static_assert(!(1 & 2) && (1 | 2));
