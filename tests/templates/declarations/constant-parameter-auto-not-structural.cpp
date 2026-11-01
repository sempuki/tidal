// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: is not a structural type
// tidal-error-gcc: because it is not structural
// tidal-error-edg: a template parameter of class type must be of
// tidal-note: The placeholder gets a class with a private data member.
struct Secret {
  constexpr Secret() {}
private:
  int hidden = 0;
};
template <auto V> struct Hold {};
Hold<Secret{}> h;
