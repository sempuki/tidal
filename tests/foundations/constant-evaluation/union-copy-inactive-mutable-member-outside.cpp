// tidal-rules: CE-22
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-gcc: reject
// tidal-error-clang: read of mutable member
// tidal-error-gcc: refers to mutable subobjects
// tidal-note: RV10-R-4. The active member fixed of shared is not mutable, so the trivial copy of the union reads no mutable part, and CE-22 sets no limit. EDG accepts. Clang and GCC reject the copy of a union that has any mutable member (Q-0449).
union Cell { mutable int count; int fixed; };
constexpr Cell shared{.fixed = 1};
constexpr Cell copy = shared;
static_assert(copy.fixed == 1);
