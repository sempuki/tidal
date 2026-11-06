// tidal-rules: PACK-17 LAM-1
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction capture_pack_by_copy_fold_generic.cpp. A generic lambda captures a pack by copy and folds over the copies.
template <class... T> auto multiplier(T... factors) {
  return [factors...](auto start) { return (factors * ... * start); };
}
int main() { return multiplier(2, 3)(7) == 42 ? 0 : 1; }
