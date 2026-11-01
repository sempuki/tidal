// tidal-rules: TSP-20
// tidal-expect: reject
// tidal-error: incomplete type
// tidal-note: Box<int> is an explicit specialization that is declared and not defined. A member access needs the complete class.
template <class T> struct Box { static constexpr int size = 1; };
template <> struct Box<int>;
int n = Box<int>::size;
