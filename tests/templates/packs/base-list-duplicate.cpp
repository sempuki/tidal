// tidal-rules: PACK-29
// tidal-expect: reject
// tidal-error-clang: specified more than once as a direct base class
// tidal-error-gcc: duplicate base type
// tidal-error-edg: duplicate base class name
// tidal-note: The pack B has the element Part twice, so the base list names Part twice.
struct Part {};
template <class... B> struct Mix : B... {};
Mix<Part, Part> bad;
