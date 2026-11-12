template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  const int c = 4;
  int a[3] = {1, 2, 3};
  static_assert(same<decltype(auto(c)), int>);
  static_assert(same<decltype(auto(a)), int*>);
  auto p = auto(a);
  return p[2] - auto{c} + 1;
}
