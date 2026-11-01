// tidal-rules: DED-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: no instance of function template
// tidal-note: The noexcept of a function type deduces a bool constant parameter. EDG finds no match (Q-0208).
template <bool B> constexpr bool nothrow(void (*)() noexcept(B)) { return B; }
void quiet() noexcept;
void loud();
static_assert(nothrow(quiet));
static_assert(!nothrow(loud));
