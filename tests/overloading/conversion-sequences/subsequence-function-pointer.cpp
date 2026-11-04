// tidal-rules: ICS-26
// tidal-expect: accept
// tidal-note: Both sequences have exact-match rank, no cv-qualifier differs, and no reference binds. Only the subsequence rule decides: after the function-to-pointer step, one sequence is the identity.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
void run() noexcept;
One take(void (*)() noexcept);
Two take(void (*)());
static_assert(same<decltype(take(run)), One>);
