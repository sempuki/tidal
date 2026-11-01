// tidal-rules: TPL-42
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is an enumeration template
// tidal-note: An unscoped member enumeration whose underlying type is the parameter T gets its enumerators after the class body. GCC rejects the definition under -pedantic-errors (Q-0199).
template <class T> struct Outer {
  enum Kind : T;
};
template <class T> enum Outer<T>::Kind : T { low = 1, high = 2 };
static_assert(Outer<char>::high == 2);
static_assert(sizeof(Outer<short>::Kind) == sizeof(short));
