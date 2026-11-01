// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function .Pinned::~Pinned
// tidal-error-edg: Pinned::~Pinned.*cannot be referenced
// tidal-note: The exemption covers only the right operand of the comma.
struct Pinned { Pinned() = default; ~Pinned() = delete; };
Pinned make();
using Result = decltype(make(), 0);
