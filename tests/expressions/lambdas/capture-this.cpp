// tidal-rules: LAM-8
// tidal-expect: run
struct Counter {
  int n = 0;
  auto adder() { return [this] { ++n; }; }
  auto copy() { return [*this]() mutable { return ++n; }; }
};
int main() {
  Counter c;
  c.adder()();
  auto k = c.copy();
  int inner = k();
  return c.n == 1 && inner == 2 ? 0 : 1;
}
