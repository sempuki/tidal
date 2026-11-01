// tidal-rules: DED-27
// tidal-expect: accept
// tidal-note: The noexcept operand would instantiate Trap<int>, whose static assertion fails. The operand is not part of substitution, and the plain function act(int) wins the call, so the operand is never substituted.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr bool value = true;
};
template <class T> int act(T) noexcept(Trap<T>::value);
long act(int);
long r = act(1);
