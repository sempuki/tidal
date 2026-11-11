template <class F> struct Box { F f; Box(F x) : f(x) {} };
template <class K> constexpr auto times(K k) {
  return Box{k};
}
int main() { return times(2).f == 2 ? 0 : 1; }
