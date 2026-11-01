// tidal-rules: ICS-38
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: ambiguous
// tidal-note: Q-0155. The literal and its null character do not fit two elements (LIST-15), so the array has no sequence. Clang forms a sequence and finds a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const char (&)[2]);
Two take(const char*);
static_assert(same<decltype(take({"ab"})), Two>);
