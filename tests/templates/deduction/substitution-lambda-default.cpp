// tidal-rules: DED-29
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to
// tidal-error-gcc: is not a class, struct, or union type
// tidal-error-edg: must be a class or namespace
// tidal-note: The lambda expression is in the default argument of a template parameter. The error in its body makes the program not valid, although a fallback template exists.
struct One {}; struct Two {};
template <class T, int = sizeof([] { typename T::missing m; })> One pick(T);
template <class... T> Two pick(T...);
void use() { pick(1); }
