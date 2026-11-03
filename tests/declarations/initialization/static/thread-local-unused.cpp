// tidal-rules: SINIT-6
// tidal-expect: run
// tidal-note: The thread_local variable is never used. It is initialized before main or never, and both are allowed.
int trace = 0;
int init() { trace = trace * 10 + 2; return 3; }
thread_local int unused = init();
int main() { trace = trace * 10 + 1; return trace == 1 || trace == 21 ? 0 : 1; }
