// tidal-rules: ALN-6
// tidal-expect: run
// tidal-sources: different-units-alignas-other.tu.cpp
// tidal-note: This unit declares the variable with alignas(32). The other unit defines it with its natural alignment. The matrix implementations accept the program.
alignas(32) extern long counter;
long bump();
int main() { return bump() == 1 ? 0 : 1; }
