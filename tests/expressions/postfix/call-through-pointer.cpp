// tidal-rules: PFX-7
// tidal-expect: run
int twice(int v) { return v * 2; }
int main() {
  int (*p)(int) = twice;
  return p(2) == 4 && (*p)(3) == 6 ? 0 : 1;
}
