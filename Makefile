.PHONY: install clean uninstall

install:
	CYTHONIZE=1 pip install .

clean:
	$(RM) -r build dist ./*.egg-info
	$(RM) -r editdistpy/damerau_osa.cpp editdistpy/levenshtein.cpp
	$(RM) -r .pytest_cache
	find . -name __pycache__ -exec rm -r {} +

uninstall:
	pip uninstall editdistpy
