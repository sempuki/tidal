// Deliberate mistake: an internal function that this unit uses and does
// not define (ODR-9, ODR-11).
#include "stats.h"
namespace { int scale(int v); }
int main() { return scale(0); }
