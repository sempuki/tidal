// tidal-rules: TSP-28 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to
// tidal-error-gcc: undefined reference to
// tidal-note: The explicit specialization of the static member template has no initializer, so it only declares Plain::size<char>.
struct Plain { template <class T> static int size; };
template <> int Plain::size<char>;
int main() { return Plain::size<char>; }
