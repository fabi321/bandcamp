# Bandcamp api

If you find any Artist/Album/Track that does not work, feel free to open an issue.

## How to use

The Rust API and Python API are identical except for search results.

```python
import bandcamp

# From URL methods first need to search for the element in question, to get the id
artist = bandcamp.artist_from_url("myrkur.bandcamp.com")

for element in artist.discography:
    print(element.title, element.item_type)

# fetch_* methods only do one api call, but require an artist and album/track id
album = bandcamp.fetch_album(artist.id, artist.discography[0].id)

for track in album.tracks:
    print(track.title)

search_results = bandcamp.search("foo")

for result in search_results:
    if isinstance(result, bandcamp.SearchResultItemAlbum):
        print("Got album", result.name)
```

### Rust search API

```rust
use bandcamp::*;

fn main() {
    let search_results = search("foo").unwrap();
    for result in search_results {
        if let SearchResultItem::Album(album) = result {
            println!("Got album {}", album.name);
        }
    }
}
```
