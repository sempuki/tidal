// tidal-rules: ICS-37
// tidal-expect: accept
// tidal-note: The two conversion functions tie on their object parameters, so the sequences from their results to the target decide.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int(); operator long(); };
long l = Source{};
static_assert(same<decltype(l), long>);
struct Small { constexpr operator short() const { return 1; } constexpr operator long() const { return 2; } };
constexpr int chosen = Small{};
static_assert(chosen == 1);
