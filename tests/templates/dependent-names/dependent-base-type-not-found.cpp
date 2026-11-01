// tidal-rules: DEP-12
// tidal-expect: reject
// tidal-error-clang: unknown type name 'size'
// tidal-error-gcc: .size. does not name a type
// tidal-error-edg: identifier "size" is undefined
template <class T> struct Base { using size = int; };
template <class T> struct Derived : Base<T> { size count = 0; };
Derived<int> d;
