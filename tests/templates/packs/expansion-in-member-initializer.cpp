// tidal-rules: PACK-13
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer reduction member_init_pack_argument.cpp. A pack expands in the expression list of a member initializer.
struct point {
  int x, y;
  point(int a, int b) : x(a), y(b) {}
};

template <class... A>
struct wrapper {
  point p;
  wrapper(A... a) : p(a...) {}
};

int main() {
  wrapper<int, int> w(2, 3);
  return w.p.x + w.p.y == 5 ? 0 : 1;
}
