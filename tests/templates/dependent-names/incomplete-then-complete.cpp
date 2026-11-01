// tidal-rules: DEP-31
// tidal-expect: reject
// tidal-error-clang: incomplete type 'Later'
// tidal-error-gcc: incomplete type .Later.
// tidal-error-edg: incomplete type "Later" is not allowed
struct Later;
template <class T> int size_of() { return sizeof(Later); }
struct Later { int a; };
int x = size_of<int>();
