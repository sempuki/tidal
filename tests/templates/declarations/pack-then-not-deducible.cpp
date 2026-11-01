// tidal-rules: TPL-35
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: cannot be used because it follows a parameter pack
// tidal-note: The type parameter after the pack has no default and no function parameter uses it. Clang and GCC accept the declaration, EDG rejects it, and TPL-35 allows both (Q-0195).
template <class... Front, class Back> int count() { return sizeof...(Front); }
