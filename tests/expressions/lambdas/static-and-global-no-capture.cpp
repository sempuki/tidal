// tidal-rules: LAM-9
// tidal-expect: run
int global = 2;
int main() {
  static int local_static = 3;
  auto g = [] { return global + local_static; };
  return g() == 5 ? 0 : 1;
}
