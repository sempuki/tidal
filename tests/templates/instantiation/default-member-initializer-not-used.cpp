// tidal-rules: INST-10
// tidal-expect: accept
template <class T> struct Box {
  int width = T::nothing;
  Box(int) : width(1) {}
};
Box<int> b(1);
