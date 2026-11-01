// tidal-rules: CNS-27
// tidal-expect: accept
// tidal-note: Answer<42> and Answer<40 + 2> name the same specialization of the concept, so the second declaration redeclares the first.
template <int N> concept Answer = N == 42;
template <class T> requires Answer<42> int f(T);
template <class T> requires Answer<40 + 2> int f(T) { return 1; }
int x = f(1);
