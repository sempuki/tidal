// tidal-rules: LAM-9
// tidal-expect: run
int main() {
  int a = 1;
  auto outer = [a] {
    auto inner = [a] { return a; };
    return inner();
  };
  return outer() == 1 ? 0 : 1;
}
