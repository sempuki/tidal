// tidal-rules: UNI-2
// tidal-expect: accept
// tidal-note: A union can carry final after its name. The union is otherwise unchanged.
union Number final { int whole; float part; };
Number n{4};
static_assert(sizeof(Number) >= sizeof(int));
