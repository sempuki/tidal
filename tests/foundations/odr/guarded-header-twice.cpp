// tidal-rules: INC-4
// tidal-expect: run
#include "pair-guarded.h"
#include "pair-guarded.h"
int main() { GuardedPair p{1, 2}; return p.second - p.first - 1; }
