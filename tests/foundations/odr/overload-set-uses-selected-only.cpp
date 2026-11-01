// tidal-rules: ODR-3
// tidal-expect: run
// tidal-note: pick(double) has no definition. The name pick selects pick(int) by the target type and by overload resolution, so only pick(int) is used, and the program links.
int pick(int v) { return v + 1; }
int pick(double v);
int apply(int (*fn)(int), int v) { return fn(v); }
int main() {
  int (*p)(int) = pick;
  int r = apply(pick, 1);
  return p(1) == 2 && r == 2 ? 0 : 1;
}
