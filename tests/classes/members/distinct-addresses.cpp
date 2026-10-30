// tidal-rules: CLS-10
// tidal-expect: run
struct Empty {};
int main() {
  Empty a, b;
  return &a != &b ? 0 : 1;
}
