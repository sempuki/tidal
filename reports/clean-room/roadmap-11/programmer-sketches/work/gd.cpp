alignas(int) unsigned char buf[4 * sizeof(int)];
int main() {
  int* d = static_cast<int*>(static_cast<void*>(buf));
  int* e = d + 2;
  return e == d ? 1 : 0;
}
