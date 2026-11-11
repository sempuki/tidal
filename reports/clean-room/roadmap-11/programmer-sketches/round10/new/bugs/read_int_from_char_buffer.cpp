// Planted bug: an int is read through a pointer into an array of char, where no int exists.
int main() {
  char buf[sizeof(int)] = {1, 0, 0, 0};
  int v = *reinterpret_cast<int*>(buf);
  return v == 1 ? 0 : 1;
}
