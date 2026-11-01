// tidal-rules: CPT-25
// tidal-expect: accept
// tidal-note: The second requirement would instantiate Trap<int>, whose static assertion fails. The first requirement fails, so the second is never checked.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr int value = 0;
};
template <class T> concept Tagged = requires { typename T::tag; Trap<T>::value; };
static_assert(!Tagged<int>);
