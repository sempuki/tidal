// tidal-rules: DEP-19
// tidal-expect: run
struct Pair { int first, second; Pair(int a, int b) : first(a), second(b) {} };
template <class T> struct Holder {
  using type = T;
  template <class U> struct wrap { U value; };
};
template <class T> int braced() { return typename Holder<T>::type{3}; }
template <class T> int parens() { return typename Holder<T>::type(4); }
template <class T> int empty() { return typename Holder<T>::type{}; }
template <class T> int member() { return typename Holder<T>::template wrap<int>{5}.value; }
template <class T> int two() { return typename Holder<T>::type(1, 2).second; }
int main() {
  if (braced<int>() != 3) return 1;
  if (parens<int>() != 4) return 2;
  if (empty<int>() != 0) return 3;
  if (member<int>() != 5) return 4;
  if (two<Pair>() != 2) return 5;
  return 0;
}
