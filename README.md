# WikiTrivia Generator

The code for generating cards for https://wikitrivia.pages.dev.

The repository for the website can be found [here](https://github.com/EdwardTheLegend/wikitrivia).

## Development

### Requirements

```
sudo apt install pbzip2 jq
sudo npm i -g wikibase-dump-filter
```

### Running

```
cargo run
```

## Notes

### Important properties

P31 : instance of
P18 : image
P1082: location

### Processing

Get all items with date claims and keep a simplified version of the item:

```
pbzip2 -d latest-all.json.bz2 -c | wikibase-dump-filter --claim 'P1082&P31&P18' --simplify > processed.jso
```

### Ranking

Rank them by fetching `contentlength` header of english wikipedia entry: `curl -I https://en.wikipedia.org/wiki/Paris`.
