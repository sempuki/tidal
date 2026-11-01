// tidal-rules: PACK-31
// tidal-expect: accept
// tidal-note: s.bits is a const lvalue, so the element of T is const int&. The reference binds to a temporary that holds the value of the bit-field.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int bits : 3; };
template <class... T> constexpr int relay(T&&... x) {
  static_assert((same<T, const int&> && ...));
  return (x + ... + 0);
}
constexpr S s{3};
static_assert(relay(s.bits) == 3);
