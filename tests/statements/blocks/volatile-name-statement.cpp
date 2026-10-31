// tidal-rules: STMT-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third). The test shows the statement is valid; it cannot observe the read.
volatile int flag = 1;
int main() { flag; return flag == 1 ? 0 : 1; }
