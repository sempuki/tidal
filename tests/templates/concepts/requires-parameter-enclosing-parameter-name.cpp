// tidal-rules: CPT-29
// tidal-expect: reject
// tidal-error-clang: declaration of 'T' shadows template parameter
// tidal-error-gcc: shadows template parameter
// tidal-error-edg: template parameter "T" may not be redeclared
template <class T> struct Box {
  template <class U> static constexpr bool named = requires (U T) { T; };
};
