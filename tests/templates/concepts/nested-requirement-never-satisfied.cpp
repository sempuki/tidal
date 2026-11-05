// tidal-rules: CPT-35
// tidal-expect: accept
// tidal-note: The nested requirements are valid and false for every argument, so Never and Empty are false for every type, and the constrained overload never wins (Q-0400).
template <class T> concept Never = requires { requires false; };
template <class T> concept Empty = requires { requires sizeof(T) == 0; };
static_assert(!Never<int> && !Empty<int> && !Empty<char>);
template <class T> requires Never<T> constexpr int pick(T) { return 1; }
template <class T> constexpr int pick(T) { return 2; }
static_assert(pick(1) == 2);
