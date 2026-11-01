// tidal-rules: DEP-37
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: specified more than once as a direct base class
// tidal-error-edg: duplicate base class name
// tidal-note: The template is never instantiated. Clang and EDG reject the base list at the definition, GCC accepts the template (Q-0247).
template <class... T> struct Mix : T..., T... {};
