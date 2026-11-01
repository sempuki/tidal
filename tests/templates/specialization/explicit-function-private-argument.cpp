// tidal-rules: TSP-29
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is private within this context
// tidal-note: The function type of the explicit specialization names a private type. GCC checks access there (Q-0204).
class Owner { struct Hidden {}; };
template <class T> int size(T) { return 0; }
template <> int size(Owner::Hidden) { return 1; }
