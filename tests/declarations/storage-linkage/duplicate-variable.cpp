// tidal-rules: LNK-6
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of .duplicated
// tidal-error-gcc: multiple definition of .duplicated
// tidal-sources: duplicate-variable.tu.cpp
int duplicated = 1;
int main() { return duplicated; }
