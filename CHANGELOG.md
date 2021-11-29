Changelog
=========

v0.1.3 (2021-11-29)
-------------------
- Fix build from source on macOS
- Handles distance calculation when one of the input strings is `None`
- Handles distance calculation when `max_distance=0`

v0.1.2 (2021-11-27)
-------------------
- Add all C++ header and source files in source distribution
- Fix `no_cythonize` path replacement
- Try cythonize by default
- Add .pyx files to source distribution

v0.1.1 (2021-11-24)
-------------------
- Handle strings as `int` array using their unicode code value for compatibility with non-English languages

v0.1.0 (2021-11-24)
-------------------
- Initial implementation
