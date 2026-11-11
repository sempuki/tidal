// tidal-rules: CE-3 CE-22
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: read of mutable member
// tidal-error-gcc: refers to mutable subobjects
// tidal-note: RV10-R-4. The implicit copy constructor of Tally reads each member of shared, and the mutable member count is not usable in constant expressions. Clang and GCC reject. EDG accepts (Q-0449).
struct Tally { mutable int count; int fixed; };
constexpr Tally shared{1, 2};
constexpr Tally copy = shared;
