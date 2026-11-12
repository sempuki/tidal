// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-error-clang: extraneous template parameter list
// tidal-error-gcc: too many template-parameter-lists
// tidal-error-edg: cannot have multiple .template
// tidal-note: RV10-7. Two template heads come before a member function declaration in a class. Every compiler rejects it.
struct Holder {
  template <class T> template <class U> void put(T, U);
};
