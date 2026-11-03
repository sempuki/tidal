// tidal-rules: TPL-35 TPL-36
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: cannot be used because it follows a parameter pack
// tidal-note: The constant pack takes its type from the type pack before it, and no function parameter gives it a value, so the declaration breaks TPL-35. EDG rejects it, and Clang and GCC accept it (Q-0195).
template <class... Types, Types... Values> int count() { return sizeof...(Values); }
