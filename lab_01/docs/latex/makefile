report:
	pdflatex --shell-escape report.tex
	biber report
	pdflatex report.tex
	pdflatex --shell-escape report.tex
	make clean

clean:
	rm -f *.aux *.bbl *.bcf *.blg *.log *.out *.run.xml *.toc *.lof *.lot *.idx *.ilg *.ind *.synctex.gz

.PHONY:
	clean report