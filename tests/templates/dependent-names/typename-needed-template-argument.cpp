// tidal-rules: DEP-19
// tidal-expect: reject
// tidal-error-clang: must be a type; did you forget 'typename'
// tidal-error-gcc: to refer to a type member of a template parameter
// tidal-error-edg: is not a type name
struct Has { using type = int; };
template <class U> struct Box { U value; };
template <class T> int size() { return sizeof(Box<T::type>); }
int x = size<Has>();
