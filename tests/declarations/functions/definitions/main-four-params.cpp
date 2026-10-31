// tidal-rules: FN-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: too many parameters \(4\) for .main
// tidal-error-gcc: takes only zero or two arguments
// tidal-note: Q-0024. Parameter lists other than () and (int, char**) are implementation-defined.
int main(int, char**, char**, char**) { return 0; }
