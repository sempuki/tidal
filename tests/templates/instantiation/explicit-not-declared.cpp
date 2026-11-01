// tidal-rules: INST-22
// tidal-expect: reject
// tidal-error-clang: does not refer to a function template
// tidal-error-gcc: .get. is not a function template
// tidal-error-edg: get is not a template
template int get<int>(int);
