// tidal-rules: DED-29
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to
// tidal-error-gcc: is not a class, struct, or union type
// tidal-error-edg: must be a class or namespace name
// tidal-note: The error is in the body of Hard<int>, outside the function type, so the program is not valid.
struct One {}; struct Two {};
template <class T> struct Hard { using type = typename T::missing; };
template <class T> One pick(T, typename Hard<T>::type* = nullptr);
Two pick(...);
void use() { pick(1); }
