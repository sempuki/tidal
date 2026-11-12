template <class T, class U> struct Pair { T first; U second; };
template <class T> using Twin = Pair<T, T>;
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  Pair p{1, 2.5};
  Twin t{3, 4};
  static_assert(same<decltype(p), Pair<int, double>>);
  static_assert(same<decltype(t), Pair<int, int>>);
  return t.second - t.first - 1 + static_cast<int>(p.second * 2) - 5;
}
