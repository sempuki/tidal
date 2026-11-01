// tidal-rules: PACK-25 PACK-17
// tidal-expect: reject
// tidal-error-clang: must precede a capture when the capture default is
// tidal-error-gcc: redundant with by-copy capture default
// tidal-error-edg: explicit capture matches default
// tidal-note: With the default =, the pack capture x... is by copy too, as for one name.
template <class... T> void keep(T... x) { auto f = [=, x...] {}; }
