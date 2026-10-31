// tidal-rules: NAM-17
// tidal-expect: accept
struct light { enum color { red, green }; };
static_assert(light::green == 1);
static_assert(light::color::green == 1);
