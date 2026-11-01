// tidal-rules: CTAD-29 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Box" matches
// tidal-note: For ('a', 'a'), each guide has U = char, and Wide<char> fails, so no guide is viable.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Box {
  template <class U> requires Wide<U> Box(U, T) {}
  template <class U> requires Wide<U> Box(T, U) {}
};
Box box('a', 'a');
