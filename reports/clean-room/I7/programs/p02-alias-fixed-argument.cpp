// CTAD-14: an alias template that fixes one argument; the fixed parameter converts.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U> struct Duo { T t; U u; constexpr Duo(T a, U b) : t(a), u(b) {} };
template <class U> using IntDuo = Duo<int, U>;
int main() {
  IntDuo d(7L, 'c');
  static_assert(same<decltype(d), Duo<int, char>>);
  return d.t == 7 && d.u == 'c' ? 0 : 1;
}
