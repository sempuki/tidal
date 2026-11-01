// tidal-rules: CNS-21
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: For int, only the defaulted constructor is eligible, so the default constructor of Box<int> is trivial and the union's default constructor is not deleted. EDG treats it as not trivial (Q-0266).
template <class T> struct Box {
  Box() requires (sizeof(T) > 1) = default;
  Box() requires (sizeof(T) == 1) {}
  int value;
};
union Slot { Box<int> box; int raw; };
Slot slot;
