// tidal-rules: TPL-3
// tidal-expect: reject
// tidal-error-clang: templates must have C\+\+ linkage
// tidal-error-gcc: specialization with C linkage
// tidal-error-edg: this declaration may not have extern
template <class T> int size(T) { return 0; }
extern "C" {
  template <> int size(char) { return 1; }
}
