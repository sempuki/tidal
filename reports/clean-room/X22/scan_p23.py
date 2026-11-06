"""Scan tests/ for P23 form 1 sites.

A site is a template-id that names a constrained class, variable or alias
template, inside the body or member list of another template, whose
arguments use a template parameter of the enclosing template. The scan is
lexical. It finds candidates; a hand review decides each one.

Usage: python3 -I scan_p23.py <tests dir>
"""
import os, re, sys
from collections import Counter

STD_CONCEPTS = set("""integral signed_integral unsigned_integral floating_point same_as
convertible_to derived_from totally_ordered equality_comparable regular semiregular
copyable movable invocable regular_invocable predicate default_initializable
constructible_from destructible assignable_from swappable copy_constructible
move_constructible three_way_comparable common_with common_reference_with
equality_comparable_with totally_ordered_with relation equivalence_relation
strict_weak_order input_iterator forward_iterator random_access_iterator range
sized_range input_range forward_range""".split())

def strip(src):
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i); i = n if j < 0 else j
        elif src.startswith('/*', i):
            j = src.find('*/', i + 2); i = n if j < 0 else j + 2; out.append(' ')
        elif c == '"' or (c == "'" and not (i > 0 and src[i-1].isalnum())):
            q = c; j = i + 1
            while j < n and src[j] != q:
                j += 2 if src[j] == '\\' else 1
            out.append(q + q); i = j + 1
        else:
            out.append(c); i += 1
    return ''.join(out)

def match_angle(s, i):
    # s[i] == '<'; return index after matching '>'
    depth = 0; paren = 0
    while i < len(s):
        c = s[i]
        if c == '(': paren += 1
        elif c == ')': paren -= 1
        elif paren == 0 and c == '<': depth += 1
        elif paren == 0 and c == '>':
            depth -= 1
            if depth == 0: return i + 1
        elif c in ';{}' and paren == 0: return -1
        i += 1
    return -1

def match_brace(s, i):
    depth = 0
    while i < len(s):
        if s[i] == '{': depth += 1
        elif s[i] == '}':
            depth -= 1
            if depth == 0: return i + 1
        i += 1
    return len(s)

ID = re.compile(r'[A-Za-z_]\w*')

def params_of(head):
    names, constrained = [], False
    inner = head[1:-1]
    parts, depth, cur = [], 0, ''
    for c in inner:
        if c in '<([': depth += 1
        if c in '>)]': depth -= 1
        if c == ',' and depth == 0: parts.append(cur); cur = ''
        else: cur += c
    if cur.strip(): parts.append(cur)
    return parts

def scan_file(path, concepts_global):
    src = strip(open(path, errors='replace').read())
    concepts = set(re.findall(r'\bconcept\s+(\w+)\s*=', src)) | concepts_global
    temps = []
    for m in re.finditer(r'\btemplate\s*<', src):
        a = m.end() - 1
        b = match_angle(src, a)
        if b < 0: continue
        head = src[a:b]
        parts = params_of(head)
        pnames, cons = [], False
        for p in parts:
            p = p.split('=')[0].strip()
            toks = ID.findall(p)
            if not toks: continue
            pnames.append(toks[-1])
            first = re.sub(r'^std::', '', p.split()[0]) if p.split() else ''
            first = first.split('<')[0]
            if first in concepts or first in STD_CONCEPTS: cons = True
        # find the entity: up to first '{' or ';' at depth 0 (skip nested templates)
        j = b; depth = 0
        while j < len(src) and not (src[j] in '{;' and depth == 0):
            if src[j] in '(': depth += 1
            if src[j] in ')': depth -= 1
            j += 1
        decl = src[b:j]
        if re.match(r'\s*template\s*<', decl):  # member template of template; handled by its own match
            continue
        if re.match(r'\s*concept\b', decl): continue
        pre_eq = decl.split('=')[0] if re.search(r'\busing\b|^[^(]*=', decl) else decl
        if re.search(r'\brequires\b', pre_eq): cons = True
        if re.search(r'\b(\w+)\s+auto\b', decl):
            for w in re.findall(r'\b(\w+)\s+auto\b', decl):
                if w in concepts or w in STD_CONCEPTS: cons = True
        km = re.search(r'(?:^|\)|\s)(struct|class|union)\s+(?:\[\[[^\]]*\]\]\s*)?(\w+)\s*(?:final\s*)?(?::[^{]*)?$', decl.strip())
        if km:
            kind, name = 'class', km.group(2)
        elif re.search(r'\busing\s+(\w+)\s*=', decl):
            kind, name = 'alias', re.search(r'\busing\s+(\w+)', decl).group(1)
        elif '(' not in decl.split('=')[0] and '=' in decl:
            kind = 'variable'; name = ID.findall(decl.split('=')[0])[-1]
        else:
            kind = 'function'
            nm = re.findall(r'(\w+)\s*\(', decl)
            name = nm[0] if nm else '?'
        body = ''
        if j < len(src) and src[j] == '{':
            e = match_brace(src, j); body = src[j:e]
        elif kind == 'alias' or kind == 'variable':
            body = ''
        temps.append(dict(kind=kind, name=name, cons=cons, params=pnames,
                          decl=decl.strip()[:120], body=body, pos=m.start()))
    return temps

def main(root):
    files = []
    for d, _, fs in os.walk(root):
        for f in fs:
            if f.endswith(('.cpp', '.h', '.hpp')):
                files.append(os.path.join(d, f))
    files.sort()
    total = Counter(); sites = []
    for path in files:
        temps = scan_file(path, set())
        callees = {t['name']: t for t in temps if t['cons'] and t['kind'] in ('class', 'variable', 'alias')}
        prog = '/programs/' in path
        for t in temps:
            total[('prog' if prog else 'test', t['kind'], t['cons'])] += 1
        for t in temps:
            if not t['body']: continue
            for cname, c in callees.items():
                if cname == t['name']: continue
                for m in re.finditer(r'\b%s\s*<' % re.escape(cname), t['body']):
                    a = m.end() - 1
                    b = match_angle(t['body'], a)
                    if b < 0: continue
                    args = t['body'][a:b]
                    if set(ID.findall(args)) & set(t['params']):
                        sites.append((path, t['name'], t['kind'], t['cons'], cname + args, c['decl']))
    print('# templates by corpus, kind, constrained')
    for k in sorted(total): print(k, total[k])
    print('# sites: file | enclosing | kind | enclosing constrained | template-id | callee declaration')
    for s in sites:
        print(' | '.join(str(x) for x in s))
    print('# site count', len(sites), 'in programs', sum(1 for s in sites if '/programs/' in s[0]))

main(sys.argv[1])

def list_callees(root):
    print('# constrained class, variable and alias templates under programs/')
    for d, _, fs in os.walk(root):
        for f in sorted(fs):
            if f.endswith(('.cpp', '.h', '.hpp')):
                path = os.path.join(d, f)
                for t in scan_file(path, set()):
                    if t['cons'] and t['kind'] != 'function':
                        print(path, '|', t['kind'], t['name'], '|', t['decl'][:90])

list_callees(os.path.join(sys.argv[1], 'programs'))
