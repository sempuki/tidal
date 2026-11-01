// tidal-rules: PACK-12 DED-35
// tidal-expect: reject
// tidal-error-clang: no matching function for call
// tidal-error-gcc: could not convert
// tidal-error-edg: no instance of function template
// tidal-note: Ts gets no elements from List<char, int>, so the parameter is List<int>, and List<char, int> does not convert to it.
template <class... Ts> struct List {};
template <class... Ts> void take(List<Ts..., int>) {}
void use() { take(List<char, int>{}); }
