// tidal-rules: OVL-9 ICS-10
// tidal-expect: run
// tidal-note: The operand of a built-in ++ or --, and the left operand of a built-in compound assignment, take a conversion function that returns a reference. Each operation changes value through the reference.
int value = 0;
struct Cell { operator int&() { return value; } };
int main() {
  Cell c;
  ++c;
  c++;
  --c;
  c--;
  ++c;
  c += 5;
  return value == 6 ? 0 : 1;
}
