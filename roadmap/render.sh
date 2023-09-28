# requires skill-tree: github.com/nikomatsakis/skill-tree

render () {
	echo "Rendering $1"
	skill-tree $1.toml output
	python3 -c "from graphviz import render; render('peer', 'png', 'output/skill-tree.peer')"
	mv output/skill-tree.peer.png "$1.png"
	rm -rf output
}

render phase-1
