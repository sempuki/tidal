// tidal-rules: LAM-7
// tidal-expect: run
int main() {
  int v = 3;
  auto g = [x = v * 2, &r = v] { r += 1; return x; };
  int got = g();
  return got == 6 && v == 4 ? 0 : 1;
}
