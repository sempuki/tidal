// tidal-rules: CNS-28
// tidal-expect: accept
template <class T> struct Box {
  int get() requires (sizeof(T) > 1);
  int get() requires (sizeof(T) <= 1);
};
template <class T> int Box<T>::get() requires (sizeof(T) > 1) { return 1; }
template <class T> int Box<T>::get() requires (sizeof(T) <= 1) { return 2; }
int x = Box<int>{}.get() + Box<char>{}.get();
