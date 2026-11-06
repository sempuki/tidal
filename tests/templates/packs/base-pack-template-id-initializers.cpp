// tidal-rules: PACK-14
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer reduction base_pack_mem_init.cpp. The base list expands leaf<I>... for a constant pack, and the member initializer expands leaf<I>{v + I}... with one initializer for each base.
template <int I> struct leaf { int value; };

template <int... I>
struct all : leaf<I>... {
  all(int v) : leaf<I>{v + I}... {}
};

int main() {
  all<0, 1> a(5);
  return a.leaf<1>::value - a.leaf<0>::value == 1 ? 0 : 1;
}
