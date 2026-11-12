int&& pass(int&& x) { return x; }  // x is an xvalue in the return statement
int main() {
  int five = 5;
  int&& r = pass(static_cast<int&&>(five));
  return r == 5 && &r == &five ? 0 : 1;
}
