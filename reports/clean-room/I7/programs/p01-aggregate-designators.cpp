// CTAD-9, CTAD-10, LIST-5: the aggregate candidate with designators and parentheses.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class K, class V> struct Entry { K key; V value; };
int main() {
  Entry a{.key = 3, .value = 2.5};
  Entry b(4L, 'x');
  static_assert(same<decltype(a), Entry<int, double>>);
  static_assert(same<decltype(b), Entry<long, char>>);
  return (a.key == 3 && b.value == 'x') ? 0 : 1;
}
