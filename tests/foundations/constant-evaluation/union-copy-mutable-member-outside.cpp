// tidal-rules: CE-22
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: read of mutable member .count.
// tidal-error-gcc: refers to mutable subobjects
// tidal-note: RV9-10. The active member count of shared is mutable, and shared began outside the evaluation, so the implicit copy constructor of Cell is not valid in the constant. Clang and GCC reject. EDG accepts (Q-0449).
union Cell { mutable int count; int fixed; };
constexpr Cell shared{1};
constexpr Cell copy = shared;
