// tidal-rules: PACK-29
// tidal-expect: reject
// tidal-error-clang: specified more than once as a direct base class
// tidal-error-gcc: duplicate base type
// tidal-error-edg: duplicate base class name
// tidal-note: The base list names Part before the expansion, and the pack B gives Part again.
struct Part {};
template <class... B> struct Mix : Part, B... {};
Mix<Part> bad;
