template <class F> struct Box { F f; };
template <class K> constexpr int times(K k) {
  Box b{k};
  return b.f;
}
int main() { return times(2) == 2 ? 0 : 1; }
