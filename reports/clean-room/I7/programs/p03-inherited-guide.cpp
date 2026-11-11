// CTAD-16: a class template that inherits constructors gets its base's guides.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base { T v; constexpr Base(T x) : v(x) {} };
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
int main() {
  Derived d(5);
  static_assert(same<decltype(d), Derived<int>>);
  return d.v == 5 ? 0 : 1;
}
