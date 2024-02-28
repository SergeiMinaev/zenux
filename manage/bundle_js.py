#!/usr/bin/env python
import os
import sys
import conf_bundle_js as conf

dirname = conf.SRC_DIR

def handle_imports(path):
    r = ''
    f = open(path)
    parent_dir = os.path.dirname(path)

    for line in f.readlines():
        if line.startswith('import '):
            mod_fname = line.split('from ')[1].strip()
            mod_fname = mod_fname[1:-2]
            new_path = os.path.normpath(os.path.join(parent_dir, mod_fname))
            if new_path not in imported:
                r += handle_imports(new_path)
                imported.append(new_path)
        else:
            r += line
    return r

for fname in conf.SRC_FILES:
    imported = []
    path = os.path.normpath(os.path.join(conf.SRC_DIR, fname))
    data = handle_imports(path)
    out = open(os.path.join(conf.OUT_DIR, fname), 'w')
    out.write(data)
