// tidal-rules: TPL-22 TPL-20
// tidal-expect: run
// tidal-note: Two function templates with one name and one function type, but heads that differ. Each explicit argument selects its own template.
template <class T> int make() { return 1; }
template <int N> int make() { return 2; }
int main() { return make<char>() == 1 && make<5>() == 2 ? 0 : 1; }
