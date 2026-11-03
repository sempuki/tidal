// tidal-rules: NS-11
// tidal-expect: reject
// tidal-error-clang: nested namespace definition cannot be .inline
// tidal-error-gcc: nested namespace definition cannot be inline
// tidal-error-edg: inline. keyword cannot be used on a nested namespace
// tidal-note: The second name of the path is inline, which is valid, and the first name is inline too.
inline namespace P::inline Q {}
