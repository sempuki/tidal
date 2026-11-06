// tidal-rules: INL-1 INL-5 TR-4 LNK-1
// tidal-expect: run
// tidal-sources: sink.tu.cpp
// tidal-note: Roadmap round 8, programmer sketch logger/main.cpp and sink.cpp (reports/clean-room/roadmap-8/programmer.md, section 1.4). The rules are the ones the report names for this program, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
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
