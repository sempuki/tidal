template <class F> struct Box { F f; };
template <class K> struct Maker {
  static constexpr auto make(K k) { return Box{k}; }
};
int main() { return Maker<int>::make(2).f == 2 ? 0 : 1; }
