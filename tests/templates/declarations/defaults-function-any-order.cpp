// tidal-rules: TPL-13
// tidal-expect: accept
// tidal-note: A function template can give a default to a parameter that a later parameter without a default follows.
template <class T = int, class U> int first(U) { return sizeof(T); }
static_assert(sizeof(first(1.0)) == sizeof(int));
