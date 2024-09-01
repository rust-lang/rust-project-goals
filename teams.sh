ls src/2024h2/*.md | while read f; do
    if [ "$f" == "src/2024h2/README.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/accepted.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/not_accepted.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/notes.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/candidates.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/flagship.md" ]; then
        continue
    fi
    g=$(basename $f)
	g=${g/.md/}
    echo $g
    grep "| Teams" $f | cut -d"|" -f3 | grep "\["
done
