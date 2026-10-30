// tidal-rules: SB-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0016. EDG 7.0 accepts a constrained auto on a structured binding.
template <class T> concept Any = true;
int a[2];
Any auto [x, y] = a;
