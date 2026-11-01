// tidal-rules: NS-6
// tidal-expect: run
namespace limits {
extern const int build_number;
static int private_count();
}
extern const int limits::build_number = 2026;
static int limits::private_count() { return 3; }
int main() {
  if (limits::build_number != 2026) return 1;
  return limits::private_count() == 3 ? 0 : 2;
}
