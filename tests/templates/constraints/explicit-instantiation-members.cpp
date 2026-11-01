// tidal-rules: CNS-15
// tidal-expect: accept
// tidal-note: The explicit instantiation of Box<int> leaves out get, whose constraint fails and whose body is invalid for int.
template <class T> struct Box {
  int get() requires (sizeof(T) > 100) { return T::missing; }
  int size() { return 1; }
};
template struct Box<int>;
