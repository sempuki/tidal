#!/usr/bin/env python3
"""X21 scanner: namespace-scope variable definitions in tests/programs/.

Heuristic. For every source file under the given directory (*.cpp, *.h),
it strips comments and literals, walks the text at namespace scope, and
lists each statement that looks like a variable definition. Each one is
classed:
  constexpr / constinit  - has that specifier
  literal                - initializer has only literals and operators
  maybe-dynamic          - anything else (manual review)
It also counts block-scope `static` variables (accessor idiom), `extern`
variable declarations, and `inline` variables, and lists the programs
(main files with a tidal-expect line) and how many units each has.
Usage: scan_programs.py TESTS_PROGRAMS_DIR
"""
import os, re, sys

root = sys.argv[1]

def strip(src):
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i); i = n if j < 0 else j
        elif src.startswith('/*', i):
            j = src.find('*/', i + 2); i = n if j < 0 else j + 2; out.append(' ')
        elif c == '"' or c == "'":
            # raw strings are rare in the corpus; treat simply
            q = c; j = i + 1
            while j < n and src[j] != q:
                j += 2 if src[j] == '\\' else 1
            out.append('0' if q == "'" else '"S"'); i = j + 1
        elif c == '#' and (i == 0 or src[i-1] == '\n'):
            j = src.find('\n', i); i = n if j < 0 else j
        else:
            out.append(c); i += 1
    return ''.join(out)

SKIP = re.compile(r'^\s*(using|typedef|template|static_assert|namespace|friend|concept|extern\s+"S"\s*$)\b')
CLASSKEY = re.compile(r'^\s*(?:\[\[[^\]]*\]\]\s*)?(?:alignas\([^)]*\)\s*)?(struct|class|union|enum)\b')
KW_LIT = {'true','false','nullptr','sizeof','alignof','static_cast','int','unsigned','long','char','double','float','short','signed','bool'}

def classify(stmt):
    s = ' '.join(stmt.split())
    if re.search(r'\bconstexpr\b', s): return 'constexpr'
    if re.search(r'\bconstinit\b', s): return 'constinit'
    m = re.search(r'=|\{', s)
    init = s[m.start():] if m else ''
    names = [w for w in re.findall(r'[A-Za-z_]\w*', init) if w not in KW_LIT and w != 'S']
    if not names: return 'literal'
    return 'maybe-dynamic'

def scan(path):
    src = strip(open(path, encoding='utf-8').read())
    res = []  # (kind, text)
    stack = []  # 'ns' or 'other'
    stmt = ''
    i, n = 0, len(src)
    def at_ns(): return all(k == 'ns' for k in stack)
    while i < n:
        c = src[i]
        if not at_ns():
            if c == '{': stack.append('other')
            elif c == '}': stack.pop()
            i += 1; continue
        if c == '{':
            head = stmt.strip()
            if re.match(r'^(inline\s+)?namespace\b', head) or re.match(r'^extern\s+"S"$', head):
                stack.append('ns'); stmt = ''; i += 1; continue
            topeq = '=' in re.sub(r'\([^()]*\)', '', head).replace('==', '').replace('<=', '').replace('>=', '').replace('!=', '')
            if CLASSKEY.match(head) and not topeq:
                # class body; then possibly declarators after '}'
                depth = 1; j = i + 1
                while depth and j < n:
                    depth += {'{':1,'}':-1}.get(src[j], 0); j += 1
                stmt = stmt + ' {...}'; i = j; continue
            if ')' in head and not topeq or SKIP.match(head) and not topeq:
                depth = 1; j = i + 1
                while depth and j < n:
                    depth += {'{':1,'}':-1}.get(src[j], 0); j += 1
                # function body or template body: statement ends
                if SKIP.match(head) and head.startswith('template') and src[j:j+20].lstrip().startswith(';'):
                    pass
                stmt = ''; i = j; continue
            # initializer braces
            depth = 1; j = i + 1
            while depth and j < n:
                depth += {'{':1,'}':-1}.get(src[j], 0); j += 1
            stmt += src[i:j]; i = j; continue
        if c == '}':
            stack.pop(); stmt = ''; i += 1; continue
        if c == ';':
            s = ' '.join(stmt.split())
            stmt = ''; i += 1
            if not s or SKIP.match(s): continue
            if CLASSKEY.match(s) and not re.search(r'\}\s*\w', s): continue
            if re.match(r'^extern\b', s) and '=' not in s:
                if '(' not in s: res.append(('extern-decl', s))
                continue
            noparen = re.sub(r'\([^()]*\)', '()', s)
            if '=' in s.replace('==','') or re.search(r'\w\s*\{', s) and not CLASSKEY.match(s):
                res.append((classify(s), s)); continue
            if '(' in s:
                # function declaration or direct initialization
                if re.search(r'\(\s*[\d"]|\(\s*(true|false|nullptr)\b', s):
                    res.append(('maybe-dynamic', s))
                continue
            res.append(('no-init', s)); continue
        stmt += c; i += 1
    return res

progs = []
allrows = []
for d, _, fs in sorted(os.walk(root)):
    for f in sorted(fs):
        if not f.endswith(('.cpp', '.h')): continue
        p = os.path.join(d, f)
        txt = open(p, encoding='utf-8').read()
        if f.endswith('.cpp') and not f.endswith('.tu.cpp') and 'tidal-expect' in txt:
            m = re.search(r'tidal-sources:(.*)', txt)
            progs.append((p, 1 + (len(m.group(1).split()) if m else 0)))
        for kind, s in scan(p):
            allrows.append((os.path.relpath(p, root), kind, s))
        allrows += [(os.path.relpath(p, root), 'block-static', m.group(0).strip())
                    for m in re.finditer(r'\n\s+static\s+(?!_assert|constexpr|inline|const\b|cast)[\w:<>,\s]+?\b\w+\s*[;({=]', strip(txt))]

print(f'programs: {len(progs)}; with more than one unit: {sum(1 for _, u in progs if u > 1)}')
for p, u in progs:
    if u > 1: print(f'  {os.path.relpath(p, root)}: {u} units')
from collections import Counter
print('counts:', dict(Counter(k for _, k, _ in allrows)))
inl = [r for r in allrows if r[1] not in ('extern-decl','block-static') and re.search(r'\binline\b', r[2])]
print('inline variables at namespace scope:', len(inl))
for f, k, s in allrows:
    if k in ('maybe-dynamic', 'extern-decl', 'block-static', 'no-init'):
        print(f'{k:14} {f}: {s[:150]}')
