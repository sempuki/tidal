// tidal-rules: LNK-6
// tidal-expect: link-fail
// tidal-sources: duplicate-variable.tu.cpp
int duplicated = 1;
int main() { return duplicated; }
