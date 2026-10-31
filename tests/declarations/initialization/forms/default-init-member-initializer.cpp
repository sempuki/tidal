// tidal-rules: INIT-9
// tidal-expect: accept
struct Settings {
  int width = 80;
  int height = 24;
};
constexpr int read() {
  Settings s;
  return s.width + s.height;
}
static_assert(read() == 104);
