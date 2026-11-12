enum Level { low, mid, high };  // no fixed underlying type: values 0 to 3
int main() {
  int raw = 9;
  Level l = static_cast<Level>(raw);
  return l == 9 ? 0 : 1;
}
