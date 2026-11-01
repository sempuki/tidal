// tidal-rules: TARG-3
// tidal-expect: accept
// tidal-note: >> closes two lists, and a > or >> operator in parentheses stays inside the list.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct Box { using type = T; };
template <int N> struct Num { static constexpr int value = N; };
static_assert(same<Box<Box<int>>::type, Box<int>>);
static_assert(Num<(3 > 2)>::value == 1);
static_assert(Num<(16 >> 2)>::value == 4);
static_assert(Num<Num<(5 > 4)>::value>::value == 1);
