// tidal-rules: CPT-12
// tidal-expect: reject
// tidal-error-clang: trailing return type must appear before trailing requires clause
// tidal-error-gcc: base operand of .->. is not a pointer
// tidal-error-edg: cannot appear at the top level
template <class T> auto get(T) requires true -> int;
