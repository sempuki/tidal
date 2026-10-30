// tidal-rules: CLS-12
// tidal-expect: run
int main() {
  struct Point { int x; int y; int sum() const { return x + y; } };
  return Point{1, 2}.sum() == 3 ? 0 : 1;
}
