// tidal-rules: DED-28 DED-29
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: cannot be used prior to
// tidal-error-gcc: is not a class, struct, or union type
// tidal-note: The default of U instantiates Bad<int>, whose body has an error. The default is substituted before the argument 1 is checked against int*, so the program is not valid. EDG checks the argument first and chooses the fallback (Q-0215).
struct One {}; struct Two {};
template <class T> struct Bad { using type = typename T::missing; };
template <class T, class U = typename Bad<T>::type> One pick(int*, T);
Two pick(...);
void use() { pick(1, 1); }
