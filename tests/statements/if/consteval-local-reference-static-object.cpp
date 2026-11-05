// tidal-rules: IF-25
// tidal-expect: run
// tidal-note: The local reference binds an object with static storage duration, so its initializer is a constant expression and runs in constant evaluation. pick takes the then branch of if consteval and returns first.
int first = 1, second = 2;
constexpr int& pick() {
  if consteval { return first; } else { return second; }
}
int main() {
  int& r = pick();
  return &r == &first ? 0 : 1;
}
