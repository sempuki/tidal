// tidal-rules: LSP-3
// tidal-expect: link-fail
extern "C" int never_defined;
int main() { return never_defined; }
