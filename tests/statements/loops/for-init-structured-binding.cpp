// tidal-rules: LOOP-4
// tidal-expect: run
struct Bounds { int low, high; };
int main() {
  int steps = 0;
  for (auto [low, high] = Bounds{2, 6}; low < high; ++low) ++steps;
  return steps == 4 ? 0 : 1;
}
