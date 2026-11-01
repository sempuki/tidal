// tidal-rules: UNI-6
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: not a valid value for type .bool.
// tidal-note: Run-time check with the sanitizer (decision 0028). The sanitizer stops the read of the inactive member because its bytes hold no valid bool. It does not report a read whose bytes happen to be a valid value.
union Cell { unsigned char code; bool flag; };
int main(int argc, char**) {
  Cell cell;
  cell.code = static_cast<unsigned char>(argc + 6);
  return cell.flag ? 0 : 1;   // undefined: flag is not active
}
