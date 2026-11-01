// tidal-rules: TPL-26
// tidal-expect: run
// tidal-note: A variable template at namespace scope and a static data member template. Each specialization is its own variable.
template <class T> T limit = T(10);
struct Table {
  template <class T> static constexpr T unit = T(1);
};
int main() {
  limit<int> = 5;
  return (limit<int> == 5 && limit<long> == 10 && Table::unit<int> == 1) ? 0 : 1;
}
