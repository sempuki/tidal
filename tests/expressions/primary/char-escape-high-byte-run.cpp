// tidal-rules: PRIM-30 PRIM-6
// tidal-expect: run
// tidal-note: A hexadecimal escape above 0x7F converts to char. Plain char is signed on the matrix targets, so '\x80' is -128 and '\xff' is -1.
int main() {
  char high = '\x80';
  char all = '\xff';
  unsigned char byte = static_cast<unsigned char>(all);
  return high == -128 && all == -1 && byte == 255 ? 0 : 1;
}
