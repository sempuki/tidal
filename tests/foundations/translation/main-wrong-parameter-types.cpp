// tidal-rules: TR-3 FN-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: first parameter of .main.
// tidal-error-gcc: first argument of .int main\(long int, char\*\*\). should be .int
// tidal-note: Q-0112. EDG 7.0 accepts main with a first parameter of type long. Clang and GCC reject it.
int main(long argc, char** argv) { return argc > 0 && argv ? 0 : 1; }
