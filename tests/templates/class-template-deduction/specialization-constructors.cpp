// tidal-rules: CTAD-5 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Shape" matches
// tidal-note: Only the primary template gives guides. The two-parameter constructors are in a partial and an explicit specialization, so no guide takes two arguments.
template <class T> struct Shape { constexpr Shape(T) {} };
template <class T> struct Shape<T*> { constexpr Shape(T*, int) {} };
template <> struct Shape<long> { constexpr Shape(long, int) {} };
int i;
Shape s(&i, 1);
