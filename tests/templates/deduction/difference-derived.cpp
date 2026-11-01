// tidal-rules: DED-15
// tidal-expect: accept
// tidal-note: The argument is a class derived from Base<int>, so T is int, for a reference and for a pointer.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Base {};
struct Derived : Base<int> {};
struct Lower : Derived {};
template <class T> Tag<T> up(Base<T>&);
template <class T> Tag<T> up_ptr(const Base<T>*);
Derived d;
Lower l;
static_assert(same<decltype(up(d)), Tag<int>>);
static_assert(same<decltype(up(l)), Tag<int>>);
static_assert(same<decltype(up_ptr(&d)), Tag<int>>);
