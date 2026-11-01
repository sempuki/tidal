// tidal-rules: INST-5
// tidal-expect: accept
template <class T> struct Box {
  T value;
  int size() { return value.size(); }
  struct Inner { int x = T::nothing; };
  template <class U> int put(U u) { return T::nothing + u; }
  int get(int extra = T::nothing) { return extra; }
  void swap() noexcept(T::nothing) {}
  static int count;
  int width = T::nothing;
  Box(int) : width(0) {}
};
template <class T> int Box<T>::count = T::nothing;
Box<int> b(1);
int x = b.get(2);
