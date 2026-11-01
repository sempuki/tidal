// tidal-rules: DED-15 DED-36
// tidal-expect: accept
// tidal-note: The parameter TT<T>& is a template template parameter applied to an argument. Derived is no specialization, so its base Base<int> gives TT and T.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <class T> struct Base {};
struct Derived : Base<int> {};
template <template <class> class TT, class T> Tag<TT<T>> find(TT<T>&);
Derived d;
static_assert(same<decltype(find(d)), Tag<Base<int>>>);
