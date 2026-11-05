// sensors.cpp: uses Fixed<16> and summarize<Q16> without making them.
#include "fixed.h"
#include "units.h"

namespace {
int taken = 0;              // internal linkage: this unit's own (TR-4)
}

// The class is still instantiated in this unit, so its constants are
// usable in a constant expression here (INST-27).
static_assert(fx::Q16::one == 65536);
static_assert(sizeof(fx::Q16) == sizeof(long long));

int sensor_serial() { return taken; }

int sensor_failures() {
  using fx::Q16;
  int bad = 0;
  taken = fx::next_serial();

  // Temperatures in degrees: -2.5, 3.25, 10, 0.75
  Q16 readings[] = {Q16::ratio(-5, 2), Q16::ratio(13, 4), Q16(10), Q16::ratio(3, 4)};
  fx::Stats<Q16> s = fx::summarize(readings, 4);
  if (!(s.low == Q16::ratio(-5, 2))) ++bad;
  if (!(s.high == Q16(10))) ++bad;
  if (!(s.total == Q16::ratio(23, 2))) ++bad;           // 11.5
  if (!(s.mean == Q16::ratio(23, 8))) ++bad;            // 2.875, exact in Q16
  if (s.count != 4) ++bad;
  if (s.low.floor() != -3) ++bad;                       // toward negative infinity
  if (s.low.round() != -3) ++bad;                       // -2.5 rounds away from zero
  if (s.mean.round() != 3) ++bad;

  // Values -2.5, 3.25, 10, 0.75 about the mean 2.875: squares of
  // 5.375, 0.375, 7.125, 2.125 sum to 84.3125, so the variance is 21.078125.
  if (!(s.variance == Q16::ratio(1349, 64))) ++bad;
  if (!(s.range() == Q16::ratio(25, 2))) ++bad;
  if (!s.within(Q16(0)) || s.within(Q16(11))) ++bad;
  int weights[] = {1, 1, 0, 2};
  if (!(fx::weighted_mean(readings, weights, 4) == Q16::ratio(9, 16))) ++bad;   // 2.25 / 4

  Q16 x = fx::percent(150);                             // 1.5
  if (!(x * x == Q16::ratio(9, 4))) ++bad;
  if (!(Q16(9) / Q16(4) == Q16::ratio(9, 4))) ++bad;
  if ((Q16(1) - Q16(3)).raw() != -2 * Q16::one) ++bad;
  return bad;
}
