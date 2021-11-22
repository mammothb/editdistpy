# MIT License
#
# Copyright (c) 2021 mmb L
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

"""Setup script."""

import os

from setuptools import Extension, setup

try:
    from Cython.Build import cythonize
except ImportError:
    cythonize = None


# https://cython.readthedocs.io/en/latest/src/userguide/source_files_and_compilation.html#distributing-cython-modules
def no_cythonize(extensions, **_ignore):
    for extension in extensions:
        sources = []
        for sfile in extension.sources:
            path, ext = os.path.splitext(sfile)
            if ext in (".pyx", ".py"):
                if extension.language == "c++":
                    ext = ".cpp"
                else:
                    ext = ".c"
                sfile = path + ext
            sources.append(sfile)
        extension.sources[:] = sources
    return extensions


ext_modules = [
    Extension(
        "editdistpy.levenshtein",
        ["editdistpy/_levenshtein.cpp", "editdistpy/levenshtein.pyx"],
        include_dirs=["./editdistpy"],
    ),
    Extension("editdistpy.damerau_osa", ["editdistpy/damerau_osa.pyx"]),
]

CYTHONIZE = bool(int(os.getenv("CYTHONIZE", "0"))) and cythonize is not None

if CYTHONIZE:
    compiler_directives = {"language_level": 3, "embedsignature": True}
    ext_modules = cythonize(ext_modules, compiler_directives=compiler_directives)
else:
    ext_modules = no_cythonize(ext_modules)

setup(
    ext_modules=ext_modules,
)
