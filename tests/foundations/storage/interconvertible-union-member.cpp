// tidal-rules: STG-16
// tidal-expect: run
union Number { int whole; float real; };
int main() {
  Number number{.whole = 4};
  int* member = reinterpret_cast<int*>(&number);
  Number* back = reinterpret_cast<Number*>(&number.whole);
  *member = 6;
  return number.whole == 6 && back == &number ? 0 : 1;
}
