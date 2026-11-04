#include "counter.h"

namespace tickets {
// Dynamically initialized during start-up (SINIT-2). The order against
// b.cpp is unspecified (SINIT-3), so this can be ticket 100 or 101.
int a_ticket = counter().take();
int ticket_of_a() { return a_ticket; }
}
