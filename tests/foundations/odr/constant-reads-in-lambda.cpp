// tidal-rules: ODR-3 ODR-2
// tidal-expect: run
struct Point { int x, y; };
int target = 3;
int& alias = target;
int main() {
  const int low = 1, high = 2;
  constexpr Point corner{4, 5};
  auto choose = [](bool first) { return first ? low : high; };
  auto read_y = [] { return corner.y; };
  auto read_alias = [] { return alias; };
  return choose(true) + read_y() + read_alias() == 1 + 5 + 3 ? 0 : 1;
}
