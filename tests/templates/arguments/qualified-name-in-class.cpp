// tidal-rules: TARG-10
// tidal-expect: reject
// tidal-error-clang: requires template arguments
// tidal-error-gcc: missing template argument list after
// tidal-error-edg: argument list for class template "lib::Box" is missing
// tidal-note: The qualified name lib::Box does not find the injected class name, so it needs an argument list in the class too.
namespace lib { template <class T> struct Box { lib::Box* self; }; }
