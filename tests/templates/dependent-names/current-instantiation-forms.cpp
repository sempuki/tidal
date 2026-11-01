// tidal-rules: DEP-15 DEP-16
// tidal-expect: run
template <class T> struct Box {
  using type = int;
  int sum() {
    Box::type a = 1;
    Box<T>::type b = 2;
    ::Box<T>::type c = 3;
    return a + b + c;
  }
};
int main() { return Box<char>{}.sum() == 6 ? 0 : 1; }
