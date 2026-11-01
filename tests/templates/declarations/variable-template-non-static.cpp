// tidal-rules: TPL-27
// tidal-expect: reject
// tidal-error-clang: cannot be declared as a template
// tidal-error-clang-18: declared as a template
// tidal-error-gcc: cannot be a member template
// tidal-error-edg: is not a valid class member template
struct Table {
  template <class T> T cell;
};
