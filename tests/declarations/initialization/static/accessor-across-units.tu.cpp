volatile int source = 7;
int load_size() { return source; }
int& table_size() {
  static int size = load_size();
  return size;
}
constinit int table_limit = 64;
