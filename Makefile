.PHONY: clean install reinstall uninstall

clean:
	$(RM) -r build dist ./*.egg-info
	$(RM) -r editdistpy/[!_]*.cpp
	$(RM) -r .pytest_cache
	find . -name __pycache__ -exec $(RM) -r {} +

test: reinstall
	pytest

install:
	pip install -v .

reinstall: clean uninstall install

uninstall:
	pip uninstall -y editdistpy
