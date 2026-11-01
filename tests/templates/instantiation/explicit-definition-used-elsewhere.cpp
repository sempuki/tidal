// tidal-rules: INST-16 INST-27
// tidal-expect: run
// tidal-sources: explicit-definition-used-elsewhere.tu.cpp
template <class T> int size(T value) { return sizeof(value); }
template <class T> struct Box { int get(); static int count; };
template <class T> int Box<T>::get() { return 2; }
template <class T> int Box<T>::count = 3;
template <class T> int zero = 4;
extern template int size(int);
extern template struct Box<int>;
extern template int zero<int>;
int main() {
  Box<int> b;
  return size(0) == sizeof(int) && b.get() == 2 && Box<int>::count == 3 && zero<int> == 4 ? 0 : 1;
}
