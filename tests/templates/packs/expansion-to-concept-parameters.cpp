// tidal-rules: PACK-11
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: pack expansion used as argument for non-pack parameter of concept
// tidal-error-gcc: pack expansion argument for non-pack parameter
// tidal-note: The concept Two has two parameters that are not packs. EDG accepts the expansion (Q-0272).
template <class A, class B> concept Two = true;
template <class... T> constexpr bool both = Two<T...>;
static_assert(both<int, char>);
