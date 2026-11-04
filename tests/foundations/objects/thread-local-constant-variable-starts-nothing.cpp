// tidal-rules: OBJ-11
// tidal-expect: run
// tidal-note: depth has constant initialization, and log_tl has a constructor. Every Clang and GCC configuration defers log_tl, and a read of depth does not start its construction. The test asserts the matrix choice (documentation/decisions/pending.md, row 4).
int made = 0;
struct Log { Log() { ++made; } };
thread_local Log log_tl;
thread_local int depth = 3;
int main() { int d = depth; return d == 3 && made == 0 ? 0 : 1; }
