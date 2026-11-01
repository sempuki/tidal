// tidal-rules: CPT-22
// tidal-expect: reject
// tidal-error-clang: deduced type 'int' does not satisfy 'LvalueRef'
// tidal-error-gcc: does not satisfy placeholder constraints
// tidal-error-edg: deduced placeholder type "int" failed
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> concept LvalueRef = same<T, T&>;
int n = 0;
LvalueRef decltype(auto) ref = n;
