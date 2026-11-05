// ledger.cpp: uses Fixed<16> and summarize<Q16>, and also Fixed<8>,
// which no explicit instantiation names, so this unit makes it.
#include "fixed.h"
#include "units.h"

namespace {
int taken = 0;
}

int ledger_serial() { return taken; }

int ledger_failures() {
  using fx::Q16;
  int bad = 0;
  taken = fx::next_serial();

  Q16 amounts[] = {fx::percent(1999), fx::percent(-500), fx::percent(250)};
  fx::Stats<Q16> s = fx::summarize(amounts, 3);
  if (!(s.low == fx::percent(-500))) ++bad;
  if (!(s.high == fx::percent(1999))) ++bad;
  if (s.total.round() != 17) ++bad;                     // 17.49
  if (s.mean.floor() != 5) ++bad;                       // 5.83

  using Q8 = fx::Fixed<8>;
  Q8 small[] = {Q8(1), Q8(2), Q8(3), Q8(6)};
  fx::Stats<Q8> t = fx::summarize(small, 4);
  if (!(t.mean == Q8(3))) ++bad;
  if (!(t.total == Q8(12))) ++bad;
  if (Q8::one != 256) ++bad;
  if (!(t.variance == Q8::ratio(7, 2))) ++bad;          // (4 + 1 + 0 + 9) / 4
  if (!(t.range() == Q8(5))) ++bad;
  if (Q8::made != 1) ++bad;                             // one ratio() call above
  int w[] = {3, 1};
  Q8 pair[] = {Q8(2), Q8(6)};
  if (!(fx::weighted_mean(pair, w, 2) == Q8(3))) ++bad;
  return bad;
}
