// tidal-rules: TPL-42
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is an enumeration template
// tidal-note: Member enumerations with a fixed underlying type, defined after the class body. GCC rejects the definitions under -pedantic-errors (Q-0199).
template <class T> struct Outer {
  enum class Kind : T;
  enum Plain : int;
};
template <class T> enum class Outer<T>::Kind : T { first, second };
template <class T> enum Outer<T>::Plain : int { width = sizeof(T) };
static_assert(static_cast<char>(Outer<char>::Kind::second) == 1);
static_assert(Outer<short>::width == sizeof(short));
