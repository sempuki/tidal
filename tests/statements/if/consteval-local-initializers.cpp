// tidal-rules: IF-25 IF-18
// tidal-expect: run
constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
enum class Mode { constant, run };
constexpr Mode mode() {
  if consteval { return Mode::constant; } else { return Mode::run; }
}
constexpr double ratio() {
  if consteval { return 1.0; } else { return 2.0; }
}
int main() {
  int local = where();
  const int kept = where();
  static int shared = where();
  const Mode m = mode();
  const double r = ratio();
  const volatile int v = where();
  return local == 2 && kept == 1 && shared == 1 && m == Mode::constant &&
                 r == 2.0 && v == 2
             ? 0
             : 1;
}
