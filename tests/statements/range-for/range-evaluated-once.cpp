// tidal-rules: RFOR-3
// tidal-expect: run
int calls = 0;
int data[3] = {1, 2, 3};
int (&get())[3] { ++calls; return data; }
int main() {
  int n = 0;
  for (int v : get()) n += v;
  return calls == 1 && n == 6 ? 0 : 1;
}
