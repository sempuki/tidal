// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function .Pinned::~Pinned
// tidal-error-edg: Pinned::~Pinned.*cannot be referenced
struct Pinned { Pinned() = default; ~Pinned() = delete; };
Pinned make();
bool quiet = noexcept(make());
