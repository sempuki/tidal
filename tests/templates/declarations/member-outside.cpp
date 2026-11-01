// tidal-rules: TPL-23
// tidal-expect: run
// tidal-note: A member function and a static data member of a class template, defined outside the class.
template <class T> struct Counter {
  T get() const;
  static T start;
  T value = start;
};
template <class T> T Counter<T>::get() const { return value; }
template <class T> T Counter<T>::start = 7;
int main() { return Counter<int>().get() == 7 ? 0 : 1; }
