// tidal-rules: INST-8
// tidal-expect: accept
template <class T> int zero = T::nothing;
using Type = decltype(zero<int>);
