// tidal-rules: TPL-19
// tidal-expect: accept
// tidal-note: The parameter T hides the global class T in the template. Outside the template, T is the class again.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct T { char c[3]; };
template <class T> struct Box { using type = T; };
static_assert(same<Box<int>::type, int>);
static_assert(sizeof(T) == 3);
