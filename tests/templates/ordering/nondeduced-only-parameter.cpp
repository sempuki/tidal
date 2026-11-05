// tidal-rules: ORD-3 ORD-15
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance
// tidal-note: U appears in the compared types only in the non-deduced position Id<U>::type. U gets no value when the first template is deduced from the second, so that direction fails, and T* does not win. unused-parameter-explicit.cpp shows the same templates with no U in the parameter types.
template <class T> struct Id { using type = T; };
template <class U, class T> int f(T, typename Id<U>::type);
template <class U, class T> int f(T*, typename Id<U>::type);
int* p = nullptr;
int use() { return f<int>(p, 1); }
