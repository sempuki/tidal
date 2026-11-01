// tidal-rules: CPT-4
// tidal-expect: reject
// tidal-error-clang: friend declaration cannot be a concept|expected member name
// tidal-error-gcc: concept definition syntax
// tidal-error-edg: expected an identifier
struct Holder {
  template <class T> friend concept Inner = true;
};
