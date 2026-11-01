// Deliberate mistake: build_number's value lives in config.cpp, so this
// unit cannot use it where a constant is needed.
#include "limits.h"
static_assert(limits::build_number == 2026);
int main() { return 0; }
