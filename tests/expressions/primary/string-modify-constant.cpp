// tidal-rules: PRIM-8
// tidal-expect: reject
// tidal-error-gcc: modification of .*abc.* is not a constant expression
// tidal-error-clang: assignment to object .*abc.* whose value is not known
// tidal-error-edg: attempt to modify const storage
constexpr int f() { char* p = const_cast<char*>("abc"); p[0] = 'x'; return 0; }
static_assert(f() == 0);
