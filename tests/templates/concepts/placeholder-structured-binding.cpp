// tidal-rules: CPT-37
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: cannot be declared with constrained 'auto'
// tidal-error-gcc: structured binding declaration cannot have constrained
// tidal-note: EDG accepts a constrained placeholder in a structured binding declaration (Q-0286).
template <class T> concept Any = true;
struct Pair { int first, second; };
void split() { Any auto [a, b] = Pair{1, 2}; }
