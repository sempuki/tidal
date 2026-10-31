// tidal-rules: TR-3
// tidal-expect: reject
int main() { auto p = &main; return p != nullptr ? 0 : 1; }
