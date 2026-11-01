// Deliberate mistake: declare the private helper here and call it.
// stats.cpp's divide_down has internal linkage (LNK-2), so this
// declaration names a different, external function that no unit defines
// (TR-4, LNK-7). The linker reports it.
#include "stats.h"
namespace stats { int divide_down(int sum, int count); }
int main() { return stats::divide_down(10, 2) - 5; }
