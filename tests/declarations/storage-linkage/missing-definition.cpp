// tidal-rules: LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .declared_only\(\)
// tidal-error-gcc: undefined reference to .declared_only\(\)
// tidal-sources: missing-definition.tu.cpp
int declared_only();
int main() { return declared_only(); }
