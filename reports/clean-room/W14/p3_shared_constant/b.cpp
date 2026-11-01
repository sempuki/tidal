#include "limits.h"

namespace limits {
// max_users is a constant expression in every unit, so it can size an
// array here (reading the value is not a use, ODR-3).
int buffer_cells_in_b() {
  int buffer[max_users / page_size] = {};
  return static_cast<int>(sizeof buffer / sizeof buffer[0]);
}
}
