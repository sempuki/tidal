// tidal-rules: CLS-11
// tidal-expect: accept
// tidal-note: An empty class with member functions and a static data member. Each target gives size 1, so an array of four has size 4.
struct Empty {
  int get() const { return 0; }
  static int count;
};
static_assert(sizeof(Empty) == 1);
static_assert(sizeof(Empty[4]) == 4);
