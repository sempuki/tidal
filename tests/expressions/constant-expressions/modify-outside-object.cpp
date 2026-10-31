// tidal-rules: CE-5
// tidal-expect: reject
// tidal-error-gcc: value of .counter. is not usable in a constant expression
int counter = 0;
constexpr int bump() { return ++counter; }
static_assert(bump() == 1);
