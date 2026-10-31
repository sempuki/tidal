// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: may not contain multiple characters
// tidal-error-gcc: multi-character literal cannot have an encoding prefix
// tidal-error-edg: too many characters in character constant
auto c = u'ab';
