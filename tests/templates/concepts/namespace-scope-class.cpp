// tidal-rules: CPT-4
// tidal-expect: reject
// tidal-error-clang: may only appear in global or namespace scope
// tidal-error-gcc: not in namespace scope
// tidal-error-edg: concept definition cannot appear
struct Holder {
  template <class T> concept Inner = true;
};
