// tidal-rules: ASN-3
// tidal-expect: run
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  int a = 0, b = 0;
  static_assert(same<decltype(a = 1), int&>);
  (a = 1) = 2;
  a = b = 3;
  return a == 3 && b == 3 ? 0 : 1;
}
