// tidal-rules: DED-29
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to
// tidal-error-gcc: is not a class, struct, or union type
// tidal-error-edg: must be a class or namespace name
// tidal-note: The error is in the body of a lambda in the function type. The body is outside the immediate context, so the error makes the program not valid.
struct One {}; struct Two {};
template <class T> One pick(T, decltype([] { typename T::missing m; }) * = nullptr);
Two pick(...);
void use() { pick(1); }
