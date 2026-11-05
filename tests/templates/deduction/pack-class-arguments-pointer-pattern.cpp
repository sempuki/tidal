// tidal-rules: DED-37
// tidal-expect: accept
// tidal-note: The expansion A*... takes each template argument of the argument's specialization, and each argument of the form X* gives A the value X.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
template <class... T> struct Tag {};
template <class... A> Tag<A...> ptrs(List<A*...>);
static_assert(same<decltype(ptrs(List<float*>{})), Tag<float>>);
static_assert(same<decltype(ptrs(List<int*, const char*>{})), Tag<int, const char>>);
