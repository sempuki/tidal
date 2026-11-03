// tidal-rules: INST-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: explicit instantiation declaration of 'value<int>' with internal linkage
// tidal-note: The variable template is declared static. Clang rejects the explicit instantiation declaration, GCC and EDG accept it (Q-0234).
template <class T> static int value = 1;
extern template int value<int>;
