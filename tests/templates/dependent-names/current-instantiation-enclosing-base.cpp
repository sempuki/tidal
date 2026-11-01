// tidal-rules: DEP-11
// tidal-expect: run
template <class T> struct Outer {
  using value_type = int;
  static int value() { return 1; }
  struct Inner;
};
int value() { return 2; }
template <class T> struct Outer<T>::Inner : Outer<T> {
  value_type get() { return value(); }
};
int main() { return Outer<char>::Inner{}.get() == 1 ? 0 : 1; }
