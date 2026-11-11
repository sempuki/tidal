// tidal-rules: PACK-14 PACK-13
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction base_pack_mem_init_under_derived.cpp. Part has a default constructor and a constructor from int. Row<0, 1> names a pack of Part bases with a template-id in parentheses, and PACK-14 gives one member initializer for each base. Row<0, 1> is itself the base of Grid, and each Part base keeps the value its initializer gives.
template <int K> struct Part {
  int value;
  Part() = default;
  Part(int v) : value(v) {}
};
template <int... K> struct Row : Part<K>... {
  Row(int start) : Part<K>(start + 10 * K)... {}
};
struct Grid : Row<0, 1> {
  Grid(int start) : Row<0, 1>(start) {}
};
int main() {
  Grid g(3);
  Part<0>& low = g;
  Part<1>& high = g;
  return (low.value == 3 && high.value == 13) ? 0 : 1;
}
