// tidal-rules: INST-22
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: declaration is incompatible
// tidal-note: The variable template has the type auto, and the explicit instantiation writes int. Clang and GCC accept it, EDG rejects it (Q-0256).
template <class T> auto zero = T(0);
template int zero<int>;
