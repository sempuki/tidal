// tidal-rules: DED-23
// tidal-expect: accept
// tidal-note: The pack P comes first in the template head and gets no value. P is empty, and T gets int from the argument.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <class... P, class T> Tag<P...> lead(T);
static_assert(same<decltype(lead(1)), Tag<>>);
