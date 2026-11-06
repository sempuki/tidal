// tidal-rules: DED-24 CNS-13
// tidal-expect: run
// tidal-note: Propose X22 (reports/clean-room/X22/report.md), finding X22-2. The default template argument Box<char> fails the constraint of Box. That is a substitution failure, so pick('a') calls the function with the ellipsis.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T, class B = Box<T>> requires Wide<T> int pick(T) { return 1; }
int pick(...) { return 2; }
int main() { return pick('a') == 2 && pick(5) == 1 ? 0 : 1; }
