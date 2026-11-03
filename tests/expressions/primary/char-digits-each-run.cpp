// tidal-rules: PRIM-31
// tidal-expect: run
// tidal-note: Each digit character minus the digit zero gives the digit's value, at run time, with each prefix.
int main() {
  const char narrow[] = "0123456789";
  const wchar_t wide[] = L"0123456789";
  const char8_t utf8[] = u8"0123456789";
  const char16_t utf16[] = u"0123456789";
  const char32_t utf32[] = U"0123456789";
  for (int i = 0; i < 10; ++i) {
    if (narrow[i] - '0' != i) return 1;
    if (wide[i] - L'0' != i) return 2;
    if (utf8[i] - u8'0' != i) return 3;
    if (utf16[i] - u'0' != i) return 4;
    if (utf32[i] - U'0' != i) return 5;
  }
  return 0;
}
