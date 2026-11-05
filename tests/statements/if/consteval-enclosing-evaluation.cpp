// tidal-rules: IF-25
// tidal-expect: run
// tidal-note: The initializer of the local double d runs in constant evaluation when the static_assert evaluates outer, because the whole call is a constant evaluation. At run time the same initializer runs at run time and takes the else branch.
constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
constexpr int outer() {
  double d = where();
  return static_cast<int>(d);
}
static_assert(outer() == 1);
int main() { return outer() == 2 ? 0 : 1; }
