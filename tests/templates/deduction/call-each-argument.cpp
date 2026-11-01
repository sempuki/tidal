// tidal-rules: DED-1
// tidal-expect: accept
// tidal-note: Each argument gives the value of one template parameter, and the specialization is the candidate. A constructor template deduces in the same way.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T, class U> Tag<void(T, U)> pair(T, U);
static_assert(same<decltype(pair(1, 'c')), Tag<void(int, char)>>);
static_assert(same<decltype(pair(1.0, 2L)), Tag<void(double, long)>>);
struct Box {
  template <class T> constexpr Box(T) : size(sizeof(T)) {}
  int size;
};
constexpr Box b('c');
static_assert(b.size == 1);
