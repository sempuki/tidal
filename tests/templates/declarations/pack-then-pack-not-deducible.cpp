// tidal-rules: TPL-35
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: cannot be used because it follows a parameter pack
// tidal-note: The second type pack follows the first, and no function parameter uses it, so the declaration breaks TPL-35. EDG rejects it, and Clang and GCC accept it (Q-0195).
template <class... Front, class... Back> int count() { return sizeof...(Front) + sizeof...(Back); }
