// tidal-rules: CNS-7
// tidal-expect: accept
template <class T> concept Valued = T::value;
struct Yes { static constexpr bool value = true; };
struct No { static constexpr bool value = false; };
static_assert(Valued<Yes> && !Valued<No> && !Valued<int>);
