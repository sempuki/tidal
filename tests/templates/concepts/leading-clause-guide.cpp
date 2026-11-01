// tidal-rules: CPT-8
// tidal-expect: accept
// tidal-note: The guide's clause fails for char, so deduction uses the guide from the constructor.
template <class T> concept Wide = sizeof(T) > 1;
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { Box(T) {} };
template <class T> requires Wide<T> Box(T) -> Box<long>;
Box narrow('a');
Box wide(1);
static_assert(same<decltype(narrow), Box<char>>);
static_assert(same<decltype(wide), Box<long>>);
