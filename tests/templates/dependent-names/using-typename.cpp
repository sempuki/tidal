// tidal-rules: DEP-23
// tidal-expect: run
template <class T> struct Base { using type = int; };
template <class T> struct Derived : Base<T> {
  using typename Base<T>::type;
  type value = 4;
};
int main() { return Derived<char>{}.value == 4 ? 0 : 1; }
