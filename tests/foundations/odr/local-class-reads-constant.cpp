// tidal-rules: ODR-3 ODR-2
// tidal-expect: run
int main() {
  struct Size { int w, h; };
  const int limit = 4;
  constexpr Size size{2, 3};
  struct Reader {
    int read() { return limit + size.h; }
  };
  return Reader().read() - 7;
}
