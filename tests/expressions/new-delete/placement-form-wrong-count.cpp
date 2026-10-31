// tidal-rules: NEW-20
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .operator new
// tidal-error-gcc: no matching function for call to .operator new
// tidal-error-edg: no instance of overloaded .*operator new
struct arena { };
void* operator new(decltype(sizeof 0), arena&);
int* make(arena& a) { return new (a, 1) int; }
