// tidal-rules: UNI-11 UNI-3
// tidal-expect: reject
// tidal-error-clang: implicitly.deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: The copy constructor of Pinned is deleted, so the copy constructor of the union is deleted too.
struct Pinned { Pinned() = default; Pinned(const Pinned&) = delete; };
union Slot { Pinned pinned; int raw; };
Slot a{};
Slot b = a;
