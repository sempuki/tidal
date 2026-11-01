// tidal-rules: INST-17
// tidal-expect: accept
template <class T> struct Box {
  T value;
  template <class U> int put(U u) { return u.size(); }
  int later();
  int size() { return value.size(); }
  int get(int extra = T::nothing) { return extra; }
};
template <> int Box<int>::size() { return 0; }
template struct Box<int>;
