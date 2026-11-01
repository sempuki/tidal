// tidal-rules: DEP-15 DEP-16
// tidal-expect: run
template <class T> struct Outer {
  struct Inner {
    using type = int;
    int sum() {
      Inner::type a = 1;
      Outer<T>::Inner::type b = 2;
      return a + b;
    }
  };
};
int main() { return Outer<char>::Inner{}.sum() == 3 ? 0 : 1; }
