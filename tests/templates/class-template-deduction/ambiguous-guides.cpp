// tidal-rules: CTAD-24
// tidal-expect: reject
// tidal-error-clang: ambiguous deduction for template arguments
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of constructor "Two" matches
// tidal-note: Both guides need one conversion and neither is better.
template <class T> struct Two { Two(T, int) {} Two(int, T) {} };
Two t(1, 2);
