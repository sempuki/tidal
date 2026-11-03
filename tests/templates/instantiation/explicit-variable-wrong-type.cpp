// tidal-rules: INST-22
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: does not match expected type 'int'
// tidal-error-edg: declaration is incompatible
// tidal-note: The explicit instantiation names the variable template's specialization with the type long, and the specialization has the type int. Clang and EDG reject it, GCC accepts it (Q-0346).
template <class T> int value = 1;
template long value<int>;
