#include "count-template.h"
template <> int digits(long) { return 19; }
int digits_elsewhere() { return digits(7L); }
