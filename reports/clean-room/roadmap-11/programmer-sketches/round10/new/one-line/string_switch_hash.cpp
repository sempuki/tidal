// A switch on a constexpr hash of a string, with case labels from the same hash.
constexpr unsigned hash(const char* s) { unsigned h = 5381; while (*s) h = h * 33 + static_cast<unsigned char>(*s++); return h; }
int code(const char* cmd) {
  switch (hash(cmd)) {
    case hash("start"): return 1;
    case hash("stop"): return 2;
    default: return 0;
  }
}
int main() { return code("stop") == 2 && code("start") == 1 && code("x") == 0 ? 0 : 1; }
