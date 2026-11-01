// tidal-rules: CPT-21 CNS-12
// tidal-expect: reject
// tidal-error-clang: no matching function for call to 'pick'
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template "pick" matches
template <class T> concept Wide = sizeof(T) > 1;
template <Wide auto N> constexpr int pick() { return 1; }
int chosen = pick<'a'>();
