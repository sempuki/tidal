// tidal-rules: LSP-1
// tidal-expect: run
extern "C" { int visible_after = 4; }
int main() { return visible_after == 4 ? 0 : 1; }
