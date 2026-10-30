// tidal-rules: LAM-4
// tidal-expect: run
int main() {
  int n = 1;
  auto g = [&n] { return n; };
  auto set = [&n] { n = 9; };
  n = 5;
  int seen = g();
  set();
  return seen == 5 && n == 9 ? 0 : 1;
}
