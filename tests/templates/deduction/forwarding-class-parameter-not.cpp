// tidal-rules: DED-8
// tidal-expect: accept
// tidal-note: T belongs to the class template, so put(T&&) takes int&&, which an lvalue cannot bind, and put(const T&) is chosen.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T> struct Box {
  One put(T&&);
  Two put(const T&);
};
Box<int> b;
int n = 0;
static_assert(same<decltype(b.put(n)), Two>);
static_assert(same<decltype(b.put(1)), One>);
