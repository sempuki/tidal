// tidal-rules: STMT-3
// tidal-expect: accept
void f(bool b) {
  if (b) goto end;
  b = !b;
end:
}
