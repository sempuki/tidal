// tidal-rules: CE-5
// tidal-expect: reject
int counter = 0;
constexpr int bump() { return ++counter; }
static_assert(bump() == 1);
