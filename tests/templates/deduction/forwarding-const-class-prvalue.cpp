// tidal-rules: DED-8
// tidal-expect: accept
// tidal-note: I5-6. A call of a function that returns const Box is a prvalue of type const Box, because a class prvalue keeps its cv-qualifiers. So T is const Box.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> fwd(T&&);
struct Box {};
const Box make();
static_assert(same<decltype(fwd(make())), Tag<const Box>>);
static_assert(same<decltype(fwd(Box{})), Tag<Box>>);
