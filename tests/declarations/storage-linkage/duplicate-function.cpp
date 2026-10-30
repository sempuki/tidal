// tidal-rules: LNK-6
// tidal-expect: link-fail
// tidal-sources: duplicate-function.tu.cpp
int duplicated() { return 1; }
int main() { return duplicated(); }
