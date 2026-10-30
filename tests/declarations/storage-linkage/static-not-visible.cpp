// tidal-rules: LNK-2
// tidal-expect: link-fail
// tidal-sources: static-not-visible.tu.cpp
int hidden();
int main() { return hidden(); }
