// tidal-rules: TSP-25 LNK-6
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of
// tidal-error-gcc: multiple definition of
// tidal-sources: explicit-not-inline.tu.cpp
// tidal-note: The template is inline and the explicit specialization is not, so the two units each define size<char>.
template <class T> inline int size(T) { return 0; }
template <> int size(char) { return 1; }
int other();
int main() { return size('a') + other() == 2 ? 0 : 1; }
