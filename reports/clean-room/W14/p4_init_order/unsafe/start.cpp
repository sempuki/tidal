// The unsafe form: a namespace-scope variable with a dynamic initializer.
volatile int seed = 100;   // volatile: reading it is never a constant
int start = seed;          // dynamic initialization (SINIT-2)
