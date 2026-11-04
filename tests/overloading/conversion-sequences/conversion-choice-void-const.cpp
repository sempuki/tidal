// tidal-rules: ICS-43
// tidal-expect: accept
// tidal-note: The classes form a chain of three, and the target is const void*. Both results convert by conversion rank, and only the preference for the base decides.
struct Top {};
struct Middle : Top {};
struct Bottom : Middle {};
Middle m;
struct Source {
  constexpr operator Bottom*() const { return nullptr; }
  constexpr operator Middle*() const { return &m; }
};
constexpr const void* p = Source{};
static_assert(p == &m);
