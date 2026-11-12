struct Counter {
  int n = 0;
  constexpr Counter bumped(this Counter self) { ++self.n; return self; }
};
constexpr Counter c{};
static_assert(c.bumped().bumped().n == 2 && c.n == 0);
int main() { return Counter{}.bumped().n - 1; }
