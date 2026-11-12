// tidal-rules: ATTR-19
// tidal-expect: run
// tidal-note: RV10-8. An attribute specifier comes before a structured binding declaration in a block and in the initializer of an if and a for. Every compiler accepts each form.
struct Pair { int a; int b; };
int main() {
  Pair s{1, 2};
  [[maybe_unused]] auto [x, y] = s;
  if ([[maybe_unused]] auto [a, b] = s; a == 1) {
    for ([[maybe_unused]] auto [c, d] = s; c < 1;) {}
    return b == 2 && x == 1 ? 0 : 1;
  }
  return 1;
}
