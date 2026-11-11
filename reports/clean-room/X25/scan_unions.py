"""Lexical scan for X25: unions with an array member.

For each .cpp or .h file under the given roots, find each union body
(named or anonymous) and report the members whose declarator has a
bound in brackets. Also report whether the file calls construct_at or
placement new, and whether it uses constexpr/consteval/static_assert.
"""
import re, sys, pathlib

def strip(text):
    text = re.sub(r'//[^\n]*', '', text)
    text = re.sub(r'/\*.*?\*/', '', text, flags=re.S)
    text = re.sub(r'"(\\.|[^"\\\n])*"', '""', text)
    return text

def unions(text):
    for m in re.finditer(r'\bunion\b[^{;()]*\{', text):
        i = m.end(); depth = 1
        while i < len(text) and depth:
            if text[i] == '{': depth += 1
            elif text[i] == '}': depth -= 1
            i += 1
        body = text[m.end():i-1]
        # top-level member declarations only
        flat = []; d = 0
        for ch in body:
            if ch == '{': d += 1
            elif ch == '}': d -= 1; continue
            if d == 0: flat.append(ch)
        decls = ''.join(flat).split(';')
        arrays = [s.strip() for s in decls
                  if re.search(r'\w\s*\[[^\]]+\]\s*(=.*)?$', s.strip())
                  and '(' not in s]
        yield text.count('\n', 0, m.start()) + 1, arrays

for root in sys.argv[1:]:
    for p in sorted(pathlib.Path(root).rglob('*')):
        if p.suffix not in ('.cpp', '.h', '.hpp'): continue
        t = strip(p.read_text(errors='replace'))
        hits = [(ln, a) for ln, a in unions(t) if a]
        if not hits: continue
        ca = 'construct_at' in t
        pn = bool(re.search(r'\bnew\s*\(', t))
        ce = bool(re.search(r'\b(constexpr|consteval|static_assert)\b', t))
        for ln, a in hits:
            print(f'{p}:{ln}: arrays={a} construct_at={ca} placement_new={pn} constant={ce}')
