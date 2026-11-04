// tidal-rules: CMP-1
// tidal-expect: accept
// tidal-flags: -Werror -Wparentheses
// tidal-note: Q-0029. No compiler warns about a chain in a dependent requirement or in a template that is never instantiated. CMP-14 asks for a warning, which is recommended practice only.
template <class T> concept Ordered = requires (T a, T b, T c) { a < b < c; };
static_assert(Ordered<int>);
template <class T> bool unused(T a, T b, T c) { return a < b < c; }
