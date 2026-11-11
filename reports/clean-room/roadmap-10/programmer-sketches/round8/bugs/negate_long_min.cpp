int main() {
  long v = -9223372036854775807L - 1;
  v = -v;
  return v == 0;
}
