// tidal-rules: FN-6
// tidal-expect: reject
// tidal-flags: -m32
// tidal-expect-edg: accept
// tidal-error-clang: first parameter of .main. .* must be of type .int
// tidal-error-gcc: first argument of .int main\(double\). should be .int
// tidal-note: Q-0024. -m32. Parameter lists other than () and (int, char**) are implementation-defined.
int main(double) { return 0; }
