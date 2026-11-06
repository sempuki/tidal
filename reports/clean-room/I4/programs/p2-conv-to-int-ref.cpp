// A class that converts to int& against an int lvalue: COND-16 step 1, lvalue int.
int store = 1;
struct R { int& operator_ref(); operator int&() { return store; } };
int main() {
  R r; int i = 5; bool c = true;
  (c ? r : i) = 7;
  return store == 7 && i == 5 ? 0 : 1;
}
