// tidal-rules: TPL-14 TPL-15
// tidal-expect: accept
// tidal-note: Each declaration on its own has a default before a parameter without one in that declaration. Counted with the earlier declarations, every parameter after the first default has a default.
template <class T, class U, class V = int> struct Triple;
template <class T, class U = long, class V> struct Triple;
template <class T = char, class U, class V> struct Triple { using types = Triple; };
static_assert(sizeof(Triple<>*) > 0);
