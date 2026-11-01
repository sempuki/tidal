// Deliberate mistake: ask for constant initialization of a value that is
// only known at run time. CINIT-1: the compiler must reject it.
volatile int seed = 100;
constinit int start = seed;
int main() { return start - 100; }
