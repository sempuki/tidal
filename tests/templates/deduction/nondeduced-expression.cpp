// tidal-rules: DED-13 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: N + 1 is an expression of N, so Fixed<3> gives N no value.
template <int N> struct Fixed {};
template <int N> void next(Fixed<N + 1>);
void use() { next(Fixed<3>{}); }
