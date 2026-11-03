// tidal-rules: UNI-15 UNI-10
// tidal-expect: run
// tidal-note: cv-qualifiers do not matter for the common initial sequence, so a const int member matches an int member.
struct Reading { int sensor; int value; };
struct Fixed { const int sensor; const int value; };
union Slot { Reading reading; Fixed fixed; };
int main() {
  Slot s{.reading = {3, 40}};
  return s.fixed.sensor == 3 && s.fixed.value == 40 ? 0 : 1;
}
