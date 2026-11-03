// tidal-rules: ASN-9
// tidal-expect: run
// tidal-note: The left operand is a subscript with an increment. The increment runs once.
int main() {
  int values[3] = {1, 2, 4};
  int i = 0;
  values[i++] |= 8;
  return i == 1 && values[0] == 9 && values[1] == 2 ? 0 : 1;
}
