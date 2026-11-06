// tidal-rules: OBJ-11
// tidal-expect: run
// tidal-note: Gap review 2026-11-05 of the text phase 10 wrote. Q-0416. A use of the constant-initialized thread_local k does not construct the thread_local c of the same unit on the baseline compilers. The review reads that the use of any non-inline thread_local of the unit starts the deferred construction.
int log_ = 0;
struct C { C() { log_ = 1; } };
thread_local int k = 5;
thread_local C c;
int main() { int v = k; return v == 5 && log_ == 0 ? 0 : 1; }
