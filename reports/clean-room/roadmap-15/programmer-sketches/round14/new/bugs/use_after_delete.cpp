struct Slot { int v; };
int main() {
  Slot* s = new Slot{5};
  Slot* alias = s;
  delete s;
  return alias->v == 5 ? 0 : 1;
}
