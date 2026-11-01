// tidal-rules: INST-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: The template has internal linkage. Clang and EDG reject the explicit instantiation declaration, GCC accepts it (Q-0234).
// tidal-error-clang: explicit instantiation declaration of 'get' with internal linkage
// tidal-error-edg: cannot refer to a specialization of static function template
template <class T> static int get(T) { return 1; }
extern template int get(int);
