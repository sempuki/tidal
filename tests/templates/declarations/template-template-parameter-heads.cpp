// tidal-rules: TPL-12
// tidal-expect: accept
// tidal-note: The heads of the template template parameters have a constant parameter and a pack. The name takes arguments for that head.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <int N> struct Fixed { static constexpr int size = N; };
template <class... T> struct List {};
template <template <int> class C> struct Sized { using type = C<4>; };
template <template <class...> class C> struct Many { using type = C<int, char>; };
static_assert(Sized<Fixed>::type::size == 4);
static_assert(same<Many<List>::type, List<int, char>>);
