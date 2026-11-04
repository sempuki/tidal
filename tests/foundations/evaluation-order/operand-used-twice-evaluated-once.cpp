// tidal-rules: SEQ-11
// tidal-expect: run
struct Pair { int first; int second; };
struct Row {
  int cells[3];
  constexpr const int* begin() const { return cells; }
  constexpr const int* end() const { return cells + 3; }
};

constexpr int increments() {
  int value = 0;
  int calls = 0;
  auto at = [&]() -> int& { ++calls; return value; };
  at() += 5;
  ++at();
  at()++;
  --at();
  at()--;
  at() *= 2;
  return calls * 100 + value;
}
static_assert(increments() == 610);

constexpr Pair make_pair(int& calls) { ++calls; return {1, 2}; }
constexpr Row make_row(int& calls) { ++calls; return {{1, 2, 3}}; }
constexpr int next(int& calls) { ++calls; return 2; }

constexpr int statements() {
  int calls = 0;
  auto [a, b] = make_pair(calls);
  int sum = a + b + a + b;
  for (int v : make_row(calls)) sum += v;
  switch (next(calls)) {
  case 0: sum += 100; break;
  case 1: sum += 200; break;
  case 2: sum += 300; break;
  }
  return calls * 1000 + sum;
}
static_assert(statements() == 3312);

int main() {
  if (increments() != 610) return 1;
  if (statements() != 3312) return 2;
  return 0;
}
