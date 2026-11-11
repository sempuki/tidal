// A header with unsigned bit-fields: each field keeps only its width.
struct Header {
  unsigned version : 3;
  unsigned flags : 5;
  unsigned length : 8;
};
int main() {
  Header h{};
  h.version = 9;   // 9 does not fit in 3 bits: the field keeps 1
  h.flags = 0x1f;
  h.length = 200;
  return h.version == 1 && h.flags == 31 && h.length == 200 ? 0 : 1;
}
