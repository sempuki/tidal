// tidal-rules: PACK-14 CTOR-15
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction base_pack_paren_mem_init_with_constructor.cpp. The pack expansion Part<K>(start + 10 * K)... gives one member initializer for each base (PACK-14). Part<0> and Part<1> are two bases, so no base is named two times, and CTOR-15 does not apply.
template <int K> struct Part {
  int value;
  Part() = default;
  Part(int v) : value(v) {}
};
template <int... K> struct Row : Part<K>... {
  Row(int start) : Part<K>(start + 10 * K)... {}
};
int main() {
  Row<0, 1> r(4);
  Part<0>& low = r;
  Part<1>& high = r;
  return (low.value == 4 && high.value == 14) ? 0 : 1;
}
