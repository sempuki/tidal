// tidal-rules: CX-1
// tidal-expect: accept
struct Settings {
  int width = 80;
  int height = 24;
};
constexpr Settings settings;
static_assert(settings.height == 24);
