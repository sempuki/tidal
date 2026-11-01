// tidal-rules: DED-6
// tidal-expect: accept
// tidal-note: A volatile int loses volatile. An array of arrays becomes a pointer to its first row. An array of const int becomes a pointer to const int, since that const is not top level.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> take(T);
volatile int v = 0;
int grid[2][3];
const int fixed[2] = {1, 2};
static_assert(same<decltype(take(v)), Tag<int>>);
static_assert(same<decltype(take(grid)), Tag<int (*)[3]>>);
static_assert(same<decltype(take(fixed)), Tag<const int*>>);
