// tidal-rules: LNK-2 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .hidden\(\)
// tidal-error-gcc: undefined reference to .hidden\(\)
// tidal-sources: static-not-visible.tu.cpp
int hidden();
int main() { return hidden(); }
