template <class F> struct Box { F f; };
template <class K> constexpr auto times(K k) {
  return Box<int>{k};
}
int main() { return times(2).f == 2 ? 0 : 1; }
