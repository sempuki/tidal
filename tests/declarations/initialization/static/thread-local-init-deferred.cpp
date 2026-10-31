// tidal-rules: SINIT-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
int init() { trace = trace * 10 + 2; return 3; }
thread_local int tl = init();
int main() { trace = trace * 10 + 1; int v = tl; return v == 3 && (trace == 12 || trace == 21) ? 0 : 1; }
