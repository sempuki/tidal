// tidal-rules: IF-25
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-note: Q-0329. GCC evaluates the initializer of a local const reference that binds a temporary as a constant. Clang evaluates it at run time.
constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
int main() {
  const int& r = where();
  return r == 2 ? 0 : 1;
}
