// tidal-rules: TPL-3
// tidal-expect: reject
// tidal-error-clang: templates must have C\+\+ linkage
// tidal-error-gcc: template with C linkage
// tidal-error-edg: this declaration may not have extern
template <class T> struct Box {};
extern "C" {
  template <class T> struct Box<T*> {};
}
