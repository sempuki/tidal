// tidal-rules: CPT-28
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Adjusts = requires (T list[3], T make(int)) {
  requires same<decltype(list), T*>;
  requires same<decltype(make), T (*)(int)>;
};
static_assert(Adjusts<int>);
