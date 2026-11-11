// tidal-rules: SW-4
// tidal-expect: run
struct Pair { int a, b; };
int main() {
  int r = 0;
  switch (auto [a, b] = Pair{1, 2}; a + b) {
    case 3: r = b; break;
    default: r = -1;
  }
  return r == 2 ? 0 : 1;
}
