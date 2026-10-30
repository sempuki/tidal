// tidal-rules: STO-3
// tidal-expect: run
int global_int;
double global_double;
int* global_pointer;
int global_array[4];
int f() {
  static int block_static;
  return block_static;
}
int main() {
  for (int v : global_array)
    if (v != 0) return 1;
  return global_int == 0 && global_double == 0.0 && global_pointer == nullptr && f() == 0 ? 0 : 2;
}
