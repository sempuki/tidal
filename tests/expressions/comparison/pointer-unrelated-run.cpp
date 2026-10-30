// tidal-rules: CMP-8
// tidal-expect: run
int a, b;
int main() {
  bool lt = &a < &b, gt = &a > &b;
  return lt != gt ? 0 : 1;
}
