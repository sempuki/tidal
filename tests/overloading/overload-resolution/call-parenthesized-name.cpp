// tidal-rules: OVL-21 OVL-2
// tidal-expect: accept
// tidal-note: (get)() is the same call as get(). The current object stays the implied object argument.
struct Box {
  int v = 3;
  constexpr int get() const { return v; }
  constexpr int use() const { return (get)() + (Box::get)(); }
};
static_assert(Box{}.use() == 6);
