// tidal-rules: LSP-1
// tidal-expect: run
extern "C" { static int local_helper() { return 5; } }
int main() { return local_helper() == 5 ? 0 : 1; }
