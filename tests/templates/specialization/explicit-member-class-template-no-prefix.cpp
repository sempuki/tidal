// tidal-rules: TSP-23
// tidal-expect: reject
// tidal-error-clang: should be empty
// tidal-error-gcc: requires .template<>. syntax
// tidal-error-edg: template argument list must match the parameter list
template <class T> struct Outer { template <class U> struct Inner { int get(); }; };
template <> template <class U> struct Outer<int>::Inner { int get(); };
template <class U> int Outer<int>::Inner<U>::get() { return 3; }
