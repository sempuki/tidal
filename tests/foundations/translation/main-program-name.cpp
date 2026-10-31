// tidal-rules: TR-9
// tidal-expect: run
int main(int argc, char** argv) {
  return argc >= 1 && argv[0] != nullptr && argv[0][0] != 0 ? 0 : 1;
}
