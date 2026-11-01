// tidal-rules: INST-16
// tidal-expect: accept
template <class T> struct Box {
  int get() { return 1; }
  int bad() { return T::nothing; }
  static int count;
};
template <class T> int Box<T>::count = 1;
template int Box<int>::get();
template int Box<int>::count;
