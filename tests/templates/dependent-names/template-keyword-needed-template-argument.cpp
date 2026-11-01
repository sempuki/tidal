// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-error-clang: must be a class template or type alias template
// tidal-error-gcc: to refer to a type member of a template parameter
// tidal-error-edg: is not a class
template <template <class> class TT> struct Holder {};
struct Has { template <class U> struct box {}; };
template <class T> struct Uses { Holder<T::box> h; };
Uses<Has> u;
