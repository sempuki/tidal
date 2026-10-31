// tidal-rules: NEW-20
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .operator new
// tidal-error-gcc: no matching function for call to .operator new
// tidal-error-edg: no instance of overloaded .*operator new
struct arena { };
struct label { };
void* operator new(decltype(sizeof 0), arena&);
int* make(label tag) { return new (tag) int; }
