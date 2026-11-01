// tidal-rules: CTAD-11
// tidal-expect: accept
// tidal-note: A guide for a member class template is declared in the class, with the same access as the member.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Outer {
  template <class T> struct In { template <class U> constexpr In(U) {} };
  template <class T> In(T*) -> In<T>;
};
int n = 0;
Outer::In i(&n);
static_assert(same<decltype(i), Outer::In<int>>);
