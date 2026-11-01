// tidal-rules: CNS-21
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: The constrained copy constructor is more constrained than the other one, which is not eligible. The copy constructor of Box<int> is trivial, so the union can be copied. EDG deletes the union's copy constructor (Q-0266).
template <class T> struct Box {
  Box() = default;
  Box(const Box&) requires (sizeof(T) > 1) = default;
  Box(const Box&) {}
  int value;
};
union Slot { Box<int> box; int raw; };
void copy(Slot& from) { Slot to = from; }
