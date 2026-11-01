// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: of non-type template parameter is not a structural type
// tidal-error-gcc: is not a valid type for a template non-type parameter
// tidal-error-edg: a template parameter of class type must be of structural
// tidal-note: A class with a private data member is a literal type but not a structural type.
class Secret {
  int hidden = 0;
public:
  constexpr Secret() = default;
};
template <Secret S> struct Use {};
