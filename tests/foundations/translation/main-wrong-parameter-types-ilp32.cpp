// tidal-rules: FN-6
// tidal-expect: reject
// tidal-flags: -m32
// tidal-expect-edg: accept
// tidal-error-clang: first parameter of .main.
// tidal-error-gcc: first argument of .int main\(long int, char\*\*\). should be .int
// tidal-note: Q-0024. -m32. EDG 7.0 accepts main with a first parameter of type long. Clang and GCC reject it.
int main(long argc, char** argv) { return argc > 0 && argv ? 0 : 1; }
