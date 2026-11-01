// tidal-rules: CNS-15
// tidal-expect: accept
// tidal-note: The body of get would be invalid for int, but get is never used, so its definition is never instantiated.
template <class T> struct Box {
  int get() requires (sizeof(T) > 100) { return T::missing; }
  int size() { return 1; }
};
Box<int> box;
int x = box.size();
