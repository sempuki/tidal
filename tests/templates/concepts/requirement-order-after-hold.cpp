// tidal-rules: CPT-25
// tidal-expect: accept
// tidal-note: The first requirement holds and the second does not. The third would instantiate Trap<int>, whose static assertion fails, so it is never checked.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr int value = 0;
};
template <class T> concept Ordered = requires (T t) { t + 1; typename T::missing; Trap<T>::value; };
static_assert(!Ordered<int>);
