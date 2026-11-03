// tidal-rules: CLS-30
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0338. During constant evaluation, the address of a standard-layout object equals the address of its first member and of its base, as at run time. EDG gives false for both comparisons. All compilers give true for an array and its first element.
struct Tag {};
struct Entry : Tag { int key; int value; };
constexpr Entry entry{};
constexpr const void* address(const void* p) { return p; }
static_assert(address(&entry) == address(&entry.key));
static_assert(address(&entry) == address(static_cast<const Tag*>(&entry)));
