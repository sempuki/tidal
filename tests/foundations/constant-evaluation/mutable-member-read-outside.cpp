// tidal-rules: CE-3
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: read of mutable member
// tidal-error-gcc: mutable .Cell::count. is not usable in a constant expression
// tidal-note: RV10-R-4. The mutable member count of shared is not usable in constant expressions, although shared is constexpr, so the read is not valid in the constant. Clang and GCC reject. EDG accepts (Q-0449).
union Cell { mutable int count; int fixed; };
constexpr Cell shared{1};
constexpr int seen = shared.count;
