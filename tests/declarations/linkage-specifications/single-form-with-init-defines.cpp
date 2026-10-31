// tidal-rules: LSP-3
// tidal-expect: run
extern "C" int with_init = 4;
int main() { return with_init == 4 ? 0 : 1; }
