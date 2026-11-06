#include "logger.h"

void report(Level level, int code) {
  log().push({level, code});
  if (level == Level::error) ++errors_seen;
}

int count_from_other_unit() { return log().count(); }
