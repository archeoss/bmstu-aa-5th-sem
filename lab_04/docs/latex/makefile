report:
	xelatex report
	biber report
	#bibtex report
	xelatex report
	xelatex report
	make clean
	mv report.pdf ../report.pdf

clean:
	rm -f *.aux *.bbl *.bcf *.blg *.log *.out *.run.xml *.toc *.lof *.lot *.idx *.ilg *.ind *.synctex.gz

.PHONY:
	clean report
