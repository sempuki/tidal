// tidal-rules: CPY-15
// tidal-expect: reject
// tidal-error-clang: must pass its first argument by reference
// tidal-error-gcc: invalid constructor
// tidal-note: The class template is never instantiated. GCC reports the error under -Wtemplate-body.
template <class T>
struct Wrapper {
  Wrapper() {}
  Wrapper(Wrapper) {}   // not a template, so it is rejected at the definition
};
