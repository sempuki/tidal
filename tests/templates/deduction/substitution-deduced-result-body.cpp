// tidal-rules: DED-29
// tidal-expect: reject
// tidal-error-clang: member reference base type .* is not a structure or union
// tidal-error-gcc: request for member .* which is of non-class type
// tidal-error-edg: expression must have class type
// tidal-note: The result type of pick calls field, whose result type comes from its body. The body is instantiated, and the error in it is outside the function type of pick, so the program is not valid.
struct Two {};
template <class T> auto field(T t) { return t.missing; }
template <class T> auto pick(T t) -> decltype(field(t));
Two pick(...);
void use() { pick(1); }
