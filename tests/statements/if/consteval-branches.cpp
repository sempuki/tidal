// tidal-rules: IF-18
// tidal-expect: run
constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
constexpr int where_not() {
  if !consteval { return 2; } else { return 1; }
}
static_assert(where() == 1);
static_assert(where_not() == 1);
int main() {
  volatile int sink = 0;
  int r = where() + where_not() + sink;
  return r == 4 ? 0 : 1;
}
