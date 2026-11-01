// tidal-rules: CTAD-17 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Strict" matches
// tidal-note: Copy initialization with = leaves out the guide from the explicit constructor, and no guide remains.
template <class T> struct Strict { explicit Strict(T) {} };
Strict s = 1;
