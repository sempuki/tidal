// tidal-rules: CNS-19 CNS-21
// tidal-expect: accept
// tidal-note: For int, the defaulted destructor is selected, so Box<int> has a trivial destructor and the union's destructor is not deleted.
template <class T> struct Box {
  ~Box() requires (sizeof(T) > 1) = default;
  ~Box() requires (sizeof(T) == 1) {}
};
union Slot { Box<int> box; int raw; };
Slot slot{};
