// tidal-rules: TARG-8
// tidal-expect: accept
// tidal-note: int() can be a function type or a value-initialized int. In a template argument the type reading wins.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct Box { using type = T; };
static_assert(same<Box<int()>::type, int (*)()> == false);
static_assert(same<Box<int()>::type, int()>);
