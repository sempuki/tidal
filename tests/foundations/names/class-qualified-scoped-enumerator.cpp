// tidal-rules: NAM-18
// tidal-expect: reject
// tidal-error-clang: no member named .green. in .light.
// tidal-error-gcc: green. is not a member of .light
// tidal-error-edg: class .light. has no member .green.
struct light { enum class color { red, green }; };
static_assert(light::color::green == light::color::green);
int read() { return static_cast<int>(light::green); }
