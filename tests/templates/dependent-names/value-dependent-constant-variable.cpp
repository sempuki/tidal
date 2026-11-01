// tidal-rules: DEP-4
// tidal-expect: run
template <int N> int size() {
  const int count = N;
  int items[count];
  return sizeof items;
}
int main() { return size<3>() == 3 * sizeof(int) && size<5>() == 5 * sizeof(int) ? 0 : 1; }
