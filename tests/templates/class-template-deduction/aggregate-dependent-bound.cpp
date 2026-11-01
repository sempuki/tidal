// tidal-rules: CTAD-10 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Row" matches
// tidal-note: The bound of the array depends on N, so the braces of the array are not elided, and the list has three elements for one member.
template <int N> struct Row { int v[N]; };
Row r{1, 2, 3};
