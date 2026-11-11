// tidal-rules: CNS-3 CNS-7
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer report section 1.5 and item 6: the companion of fold-atomic.cpp, whose reject a front end can give on a grammar error. The two folds are valid requires clauses. For a pack with a type of 100 bytes or more, only the first constraint holds, so the call has one viable function and no ambiguity.
template <class T> concept Big = sizeof(T) > 1;
template <class T> concept BigBounded = Big<T> && sizeof(T) < 100;
template <class... T> requires (Big<T> && ...) constexpr int pick() { return 1; }
template <class... T> requires (BigBounded<T> && ...) constexpr int pick() { return 2; }
struct Huge { char bytes[128]; };
int main() { return pick<Huge, double>() == 1 ? 0 : 1; }
