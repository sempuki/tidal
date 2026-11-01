// tidal-rules: DEP-4
// tidal-expect: run
template <int N> int last() {
  int items[N - 1] = {};
  return sizeof items / sizeof items[0];
}
int main() { return last<5>() == 4 ? 0 : 1; }
