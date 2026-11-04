// tidal-rules: FN-6
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The matrix runs no program built with -m32, so the test checks acceptance only.
int main(int argc, char** argv, char** envp) { return argv && envp ? 0 : 1; }
