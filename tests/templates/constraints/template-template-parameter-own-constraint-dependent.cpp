// tidal-rules: CNS-14
// tidal-expect: accept
// tidal-note: Propose X22 (reports/clean-room/X22/report.md), finding X22-5. The template template parameter requires Wide, and the argument Box has no constraint. In the dependent TT<T>, only the argument's constraints are checked, so Use<Box, char> is valid.
template <class T> concept Wide = sizeof(T) > 1;
template <template <Wide> class TT, class T> struct Use { TT<T> m; };
template <class T> struct Box {};
Use<Box, char> u;
int main() {}
