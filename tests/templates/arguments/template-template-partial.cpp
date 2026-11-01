// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-note: The argument template Kind has a partial specialization for pointers. A use of K with a pointer selects it.
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> struct Kind<T*> { static constexpr int value = 1; };
template <template <class> class K> struct Use {
  static constexpr int plain = K<int>::value;
  static constexpr int pointer = K<int*>::value;
};
static_assert(Use<Kind>::plain == 0 && Use<Kind>::pointer == 1);
