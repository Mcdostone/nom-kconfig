
test:
	rm -rf rust-nom-kconfig_*
	DEBFULLNAME="Yann Prono" DEBEMAIL="yann.prono@telecomnancy.net" debcargo package nom-kconfig 0.9.0 --config test.toml