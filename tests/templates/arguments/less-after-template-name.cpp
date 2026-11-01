// tidal-rules: TARG-1
// tidal-expect: accept
// tidal-note: Lookup finds a function template and a variable template, so each < starts an argument list.
template <class T> constexpr int size_of() { return sizeof(T); }
template <class T> constexpr int width = sizeof(T) * 8;
static_assert(size_of<char>() == 1);
static_assert(width<char> == 8);
