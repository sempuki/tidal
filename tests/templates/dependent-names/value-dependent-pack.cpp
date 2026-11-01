// tidal-rules: DEP-4
// tidal-expect: run
template <class... T> int by_count() { int items[sizeof...(T) + 1]; return sizeof items; }
template <int... N> int by_fold() { int items[(N + ...)]; return sizeof items; }
int main() {
  return by_count<int, char>() == 3 * sizeof(int) && by_fold<1, 2>() == 3 * sizeof(int) ? 0 : 1;
}
