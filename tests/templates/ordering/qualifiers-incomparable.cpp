// tidal-rules: ORD-11 ORD-15
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance
// tidal-note: Both parameters become const volatile int&, so the sequences tie. The referred types const T and volatile T each have a qualifier that the other lacks. Neither set of qualifiers is a proper superset of the other, so ORD-11 removes no direction, and neither template is more specialized.
template <class T> int f(const T&);
template <class T> int f(volatile T&);
const volatile int cv = 0;
int use() { return f(cv); }
