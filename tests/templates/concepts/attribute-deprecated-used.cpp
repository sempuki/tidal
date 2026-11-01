// tidal-rules: CPT-3
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-note: The attribute deprecated appertains to the concept Old. A use of Old is valid, and Clang and GCC warn that Old is deprecated. Clang 18 rejects an attribute after the name of a concept (Q-0270).
template <class T> concept Old [[deprecated("use New")]] = sizeof(T) > 0;
static_assert(Old<int>);
