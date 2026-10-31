// tidal-rules: INC-5
// tidal-expect: run
#include "once-after-text.h"
#include "once-after-text.h"
int main() { BeforeOnce b{1}; AfterOnce a{1}; return b.v - a.v; }
