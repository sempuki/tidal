#include "logger.h"

int main() {
  report(Level::info, 1);
  report(Level::error, 2);
  log().push({.level = Level::debug, .code = 3});
  for (int i = 0; i < 10; ++i) report(Level::info, 10 + i);
  if (count_from_other_unit() != max_entries) return 1;
  if (log().newest().code != 19) return 2;
  return errors_seen == 1 ? 0 : 3;
}
