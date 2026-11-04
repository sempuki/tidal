// tidal-rules: OBJ-11
// tidal-expect: run
// tidal-note: The thread uses only clock_tl. The implementation can construct both thread_local objects of the unit before the thread's first statement, or defer them. A deferred construction happens before the first use of any thread_local variable of the unit, so log_tl exists after the read of clock_tl. Every Clang and GCC configuration defers the construction and then constructs both objects.
int made = 0;
struct Log { Log() { ++made; } };
struct Clock { int tick; Clock() : tick(5) { ++made; } };
thread_local Log log_tl;
thread_local Clock clock_tl;
int main() { int tick = clock_tl.tick; return tick == 5 && made == 2 ? 0 : 1; }
