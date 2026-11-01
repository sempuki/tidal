// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: variable .p. with type .auto. has incompatible initializer of type .<overloaded function type>.
// tidal-error-gcc: unable to deduce .auto. from .& pair<int>.
// tidal-error-edg: cannot deduce "auto" type
// tidal-note: The explicit argument fixes T but not U, and there is no target type to deduce U from.
template <class T, class U> int pair(T, U);
auto p = &pair<int>;
