// tidal-rules: CPT-3
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-note: Clang 18 rejects an attribute after the name of a concept (Q-0270).
template <class T> concept Plain [[deprecated]] = true;
template <class T> concept Other = sizeof(T) > 0;
static_assert(Other<int>);
