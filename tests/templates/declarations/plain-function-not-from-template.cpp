// tidal-rules: TPL-45 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .twice\(int\)
// tidal-error-gcc: undefined reference to .twice\(int\)
// tidal-note: The plain function twice(int) is declared and not defined. The template does not supply its definition.
template <class T> int twice(T v) { return v * 2; }
int twice(int);
int main() { return twice(3); }
