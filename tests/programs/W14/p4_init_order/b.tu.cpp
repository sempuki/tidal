#include "counter.h"

namespace tickets {
int b_ticket = counter().take();
int ticket_of_b() { return b_ticket; }
}
