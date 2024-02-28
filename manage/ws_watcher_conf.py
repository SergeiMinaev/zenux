import os
from os.path import join


BASE_DIR = os.environ['ZENUX_HOME']


PATHS = [
    {
        'dir': join(BASE_DIR, 'front/js'),
    },
    {
        'dir': join(BASE_DIR, 'front/templates/core'),
    },
    {
        'dir': join(BASE_DIR, 'front/style'),
        'onchange': {
            'cmds': [
                './bundle_css.py',
            ],
        },
    },
    {
        'dir': join(BASE_DIR, 'front/ext'),
    },
]
