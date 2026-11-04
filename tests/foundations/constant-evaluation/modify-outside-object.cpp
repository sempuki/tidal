// tidal-rules: CE-5
// tidal-expect: reject
// tidal-error-gcc: value of .counter. is not usable in a constant expression
// tidal-error-clang: cannot modify an object that is visible outside that expression
// tidal-error-edg: attempt to access run-time storage
int counter = 0;
constexpr int bump() { return ++counter; }
static_assert(bump() == 1);
