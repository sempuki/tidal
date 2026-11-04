// tidal-rules: VOL-6
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: VOL-6 makes the read through int* undefined, so the test checks only that the program compiles. No sanitizer in the matrix stops the run. constant-read-through-plain.cpp tests the same read in constant evaluation. The front end's interpreter must stop the run with VOL-6 (decision 0053).
volatile int v = 3;
int main() { int* p = const_cast<int*>(&v); return *p == 3 || *p != 3 ? 0 : 1; }
