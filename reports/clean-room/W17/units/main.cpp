// main.cpp: runs the two units and checks what they share.
#include "fixed.h"
#include "units.h"

int main() {
  int bad = 0;
  bad += sensor_failures();
  bad += ledger_failures();
  // One inline variable for the program: three calls of summarize.
  if (fx::summaries != 3) ++bad;
  // One static local in next_serial for the program (INL-5).
  if (sensor_serial() != 1 || ledger_serial() != 2) ++bad;
  if (fx::next_serial() != 3) ++bad;
  // One Fixed<16>::made for the program, from instances.cpp. sensors.cpp
  // calls ratio() 11 times and percent() 1 time; ledger.cpp calls
  // percent() 5 times. main reads the count before its own call.
  if (fx::Q16::made != 17) ++bad;
  // main also uses the library.
  fx::Q16 half = fx::percent(50);
  if (!(half + half == fx::Q16(1))) ++bad;
  return bad == 0 ? 0 : 1;
}
