// tidal-rules: TSP-29
// tidal-expect: accept
// tidal-note: The argument list of the explicit specialization names a private type of Owner.
class Owner { struct Hidden {}; };
template <class T> struct Box { static constexpr int value = 0; };
template <> struct Box<Owner::Hidden> { static constexpr int value = 1; };
