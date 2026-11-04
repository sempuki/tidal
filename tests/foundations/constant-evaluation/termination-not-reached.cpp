// tidal-rules: CE-8
// tidal-expect: run
constexpr int check(int v) noexcept {
  if (v < 0) throw v;
  return v;
}
static_assert(check(4) == 4);
int main(int argc, char**) { return check(argc) == argc ? 0 : 1; }
