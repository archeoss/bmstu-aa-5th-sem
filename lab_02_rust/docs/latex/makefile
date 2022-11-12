report:
	pdflatex report
	bibtex report
	pdflatex report
	pdflatex report
	make clean

clean:
	rm -f *.aux *.bbl *.bcf *.blg *.log *.out *.run.xml *.toc *.lof *.lot *.idx *.ilg *.ind *.synctex.gz

.PHONY:
	clean report
