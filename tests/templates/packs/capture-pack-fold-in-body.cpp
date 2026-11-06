// tidal-rules: PACK-17
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction capture_pack_by_copy_fold.cpp. A lambda captures a function parameter pack by copy and folds over the copies with its own parameter.
template <class... T> auto adder(T... parts) {
  return [parts...](int base) { return (parts + ... + base); };
}
int main() { return adder(10, 20, 30)(4) == 64 ? 0 : 1; }
