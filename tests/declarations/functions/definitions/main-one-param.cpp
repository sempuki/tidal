// tidal-rules: FN-6
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: takes only zero or two arguments
// tidal-note: Q-0024. Parameter lists other than () and (int, char**) are implementation-defined.
int main(int) { return 0; }
