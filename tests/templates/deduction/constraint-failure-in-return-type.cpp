// tidal-rules: DED-24 CNS-13
// tidal-expect: run
// tidal-note: Propose X22 (reports/clean-room/X22/report.md), finding X22-2. Box<char> in the return type fails the constraint of Box after deduction. That is a substitution failure, so make('a') calls the function with the ellipsis.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> Box<T> make(T t) { return {t}; }
int make(...) { return 7; }
int main() { return make('a') == 7 && make(5).v == 5 ? 0 : 1; }
