// tidal-rules: DEP-15 DEP-16
// tidal-expect: run
template <class T> struct Box {};
template <class T> struct Box<T*> {
  using type = int;
  int sum() {
    Box<T*>::type a = 1;
    Box::type b = 2;
    return a + b;
  }
};
int main() { return Box<char*>{}.sum() == 3 ? 0 : 1; }
