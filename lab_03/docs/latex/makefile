report:
	pdflatex --shell-escape report
	biber report
	pdflatex --shell-escape report
	pdflatex --shell-escape report
	make clean
	mv report.pdf ../report.pdf

clean:
	rm -f *.aux *.bbl *.bcf *.blg *.log *.out *.run.xml *.toc *.lof *.lot *.idx *.ilg *.ind *.synctex.gz

.PHONY:
	clean report