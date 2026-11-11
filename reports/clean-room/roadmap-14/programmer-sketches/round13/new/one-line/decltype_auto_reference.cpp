// decltype(auto) keeps a reference for a parenthesized member.
struct S { int v = 1; decltype(auto) ref() { return (v); } decltype(auto) val() { return v; } };
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  S s;
  static_assert(same<decltype(s.ref()), int&> && same<decltype(s.val()), int>);
  s.ref() = 7;
  return s.v - 7;
}
