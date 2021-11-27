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
import sys
from pathlib import Path

from setuptools import Extension, setup

try:
    from Cython.Build import cythonize
except ImportError:
    cythonize = None

PROJ_DIR = Path(__file__).resolve().parent
NAME = "editdistpy"
VERSION = "0.1.3rc1"
DESCRIPTION = "Fast Levenshtein and Damerau optimal string alignment algorithms."
with open(PROJ_DIR / "README.md", "r", encoding="utf-8") as infile:
    LONG_DESCRIPTION = infile.read()
LONG_DESCRIPTION_CONTENT_TYPE = "text/markdown"
AUTHOR = "mmb L"
URL = "https://github.com/mammothb/editdistpy"
LICENSE = "MIT"
PROJECT_URLS = {
    "Documentation": "https://github.com/mammothb/editdistpy",
    "Changelog": "https://github.com/mammothb/editdistpy/blob/master/CHANGELOG.md",
}


def no_cythonize(extensions, **_ignore):
    """Use generated .cpp files insted of building
    from: https://cython.readthedocs.io/en/latest/src/userguide/source_files_and_compilation.html#distributing-cython-modules
    """
    for extension in extensions:
        sources = []
        for sfile in map(Path, extension.sources):
            if sfile.suffix in (".pyx", ".py"):
                if extension.language == "c++":
                    suffix = ".cpp"
                else:
                    suffix = ".c"
                sfile = sfile.with_suffix(suffix)
            sources.append(str(sfile))
        extension.sources[:] = sources
    return extensions


extra_compiler_args = []
if sys.platform == "darwin":
    extra_compiler_args = ["stdlib=libc++"]

ext_modules = [
    Extension(
        "editdistpy.levenshtein",
        [
            "editdistpy/_levenshtein.cpp",
            "editdistpy/_helpers.cpp",
            "editdistpy/levenshtein.pyx",
        ],
        include_dirs=["./editdistpy"],
        language="c++",
        extra_compiler_args=extra_compiler_args,
    ),
    Extension(
        "editdistpy.damerau_osa",
        [
            "editdistpy/_damerau_osa.cpp",
            "editdistpy/_helpers.cpp",
            "editdistpy/damerau_osa.pyx",
        ],
        include_dirs=["./editdistpy"],
        language="c++",
        extra_compiler_args=extra_compiler_args,
    ),
]

NO_CYTHONIZE = "NO_CYTHONIZE" in os.environ or cythonize is None

if NO_CYTHONIZE:
    ext_modules = no_cythonize(ext_modules)
else:
    compiler_directives = {"language_level": 3, "embedsignature": True}
    ext_modules = cythonize(ext_modules, compiler_directives=compiler_directives)

setup(
    name=NAME,
    version=VERSION,
    description=DESCRIPTION,
    long_description=LONG_DESCRIPTION,
    long_description_content_type=LONG_DESCRIPTION_CONTENT_TYPE,
    author=AUTHOR,
    url=URL,
    project_urls=PROJECT_URLS,
    keywords=["edit distance", "levenshtein", "damerau"],
    license=LICENSE,
    classifiers=[
        "Development Status :: 4 - Beta",
        "License :: OSI Approved :: MIT License",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "Natural Language :: English",
        "Operating System :: Microsoft :: Windows",
        "Operating System :: POSIX",
        "Operating System :: Unix",
        "Operating System :: MacOS",
        "Programming Language :: C++",
        "Programming Language :: Cython",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: Implementation :: CPython",
    ],
    zip_safe=False,
    python_requires=">=3.6",
    include_package_data=True,
    package_dir={"editdistpy": "editdistpy"},
    ext_modules=ext_modules,
)
