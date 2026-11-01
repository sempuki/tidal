// tidal-rules: PACK-26
// tidal-expect: accept
// tidal-note: U is a pack whose element i has the constraint Same<T_i>, and W is a pack whose element i takes a constant of type T_i. With T = {int, char}, each member template takes two arguments.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class A, class B> concept Same = same<A, B>;
template <class... T> struct Outer {
  template <Same<T>... U> struct Typed { static constexpr int size = sizeof...(U); };
  template <template <T> class... W> struct Shaped { static constexpr int size = sizeof...(W); };
};
template <int> struct ByInt {};
template <char> struct ByChar {};
static_assert(Outer<int, char>::Typed<int, char>::size == 2);
static_assert(Outer<int, char>::Shaped<ByInt, ByChar>::size == 2);
