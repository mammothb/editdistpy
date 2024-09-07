# MIT License
#
# Copyright (c) 2024 mmb L
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
import platform
import sys
from sysconfig import get_config_vars

from Cython.Build import cythonize
from pkg_resources import parse_version
from setuptools import Extension, setup


def is_platform_mac():
    return sys.platform == "darwin"


extra_compile_args = []
extra_link_args = []
# Adapted from https://github.com/pandas-dev/pandas/blob/1423ef0f917220682382d478761bf31315a197ef/setup.py#L348
if is_platform_mac():
    if "MACOSX_DEPLOYMENT_TARGET" not in os.environ:
        current_system = platform.mac_ver()[0]
        python_target = get_config_vars().get(
            "MACOSX_DEPLOYMENT_TARGET", current_system
        )
        target_macos_version = "10.9"
        parsed_macos_version = parse_version(target_macos_version)
        if (
            parse_version(str(python_target))
            < parsed_macos_version
            <= parse_version(current_system)
        ):
            os.environ["MACOSX_DEPLOYMENT_TARGET"] = target_macos_version

    extra_compile_args = ["-std=c++11"]
    extra_link_args = ["-stdlib=libc++"]


ext_modules = [
    Extension(
        "editdistpy.levenshtein",
        [
            "editdistpy/_levenshtein.cpp",
            "editdistpy/levenshtein.pyx",
        ],
        include_dirs=["./editdistpy"],
        language="c++",
        extra_compile_args=extra_compile_args,
        extra_link_args=extra_link_args,
    ),
    Extension(
        "editdistpy.damerau_osa",
        [
            "editdistpy/_damerau_osa.cpp",
            "editdistpy/damerau_osa.pyx",
        ],
        include_dirs=["./editdistpy"],
        language="c++",
        extra_compile_args=extra_compile_args,
        extra_link_args=extra_link_args,
    ),
]
ext_modules = cythonize(ext_modules, compiler_directives={"language_level": 3})
setup(ext_modules=ext_modules)
