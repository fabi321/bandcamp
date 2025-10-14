from __future__ import annotations

from datetime import datetime
from typing import Optional

class Image:
    def get_image_id(self) -> Optional[int]: ...
    def get_url(self) -> Optional[str]: ...
    def get_with_resolution(self, resolution: ImageResolution) -> Optional[str]: ...

class AlbumImage:
    def get_image_id(self) -> Optional[int]: ...
    def get_url(self) -> Optional[str]: ...
    def get_with_resolution(self, resolution: ImageResolution) -> Optional[str]: ...

class AlbumType:
    Album: AlbumType
    Track: AlbumType

class ArtistDiscographyEntryType:
    Album: ArtistDiscographyEntryType
    Track: ArtistDiscographyEntryType

async def fetch_artist(artist_id: int) -> Artist: ...
async def fetch_album(artist_id: int, album_id: int) -> Album: ...
async def fetch_track(artist_id: int, track_id: int) -> Album: ...
async def search(
    query: str,
) -> list[
    SearchResultItemAlbum
    | SearchResultItemArtist
    | SearchResultItemTrack
    | SearchResultItemFan
]: ...
async def album_from_url(url: str) -> Album: ...
async def artist_from_url(url: str) -> Artist: ...
async def track_from_url(url: str) -> Album: ...

# DO NOT EDIT BELOW THIS LINE! ----------------
class Album:
    """Single track or album"""

    @property
    def id(self) -> int:
        """Track or album id"""

    @property
    def item_type(self) -> AlbumType:
        """Type of item"""

    @property
    def title(self) -> str: ...
    @property
    def url(self) -> str: ...
    @property
    def image(self) -> AlbumImage: ...
    @property
    def band(self) -> AlbumBand:
        """Band this belongs to"""

    @property
    def album_id(self) -> Optional[int]:
        """For a track belonging to an album"""

    @property
    def album_title(self) -> Optional[str]:
        """For a track belonging to an album"""

    @property
    def about(self) -> Optional[str]:
        """About text for an album or track, explaining it a bit"""

    @property
    def credits(self) -> Optional[str]:
        """Credits for the album or track"""

    @property
    def tracks(self) -> list[AlbumTrack]: ...
    @property
    def featured_track(self) -> Optional[int]:
        """The track id of the featured track, if any"""

    @property
    def release_date(self) -> datetime: ...
    @property
    def tags(self) -> list[AlbumTag]: ...
    @property
    def free_download(self) -> bool: ...
    @property
    def is_preorder(self) -> bool: ...
    @property
    def purchase_options(self) -> PurchaseOptions: ...
    @property
    def label(self) -> Optional[str]: ...
    @property
    def label_id(self) -> Optional[int]: ...
    @property
    def num_downloadable_tracks(self) -> int: ...
    @property
    def merch_sold_out(self) -> Optional[bool]: ...
    @property
    def streaming_limit(self) -> Optional[int]:
        """
        How often a user can stream a track before being prompted to buy it,
        see https://get.bandcamp.help/hc/en-us/articles/23020694060183-What-are-streaming-limits-on-Bandcamp
        """

class AlbumBand:
    @property
    def id(self) -> int: ...
    @property
    def name(self) -> str: ...
    @property
    def image(self) -> Image: ...
    @property
    def bio(self) -> Optional[str]: ...
    @property
    def location(self) -> Optional[str]: ...

class AlbumTag:
    @property
    def name(self) -> str: ...
    @property
    def normalized_name(self) -> str: ...
    @property
    def url(self) -> str: ...
    @property
    def is_location(self) -> bool: ...
    @property
    def location_id(self) -> Optional[int]: ...
    @property
    def geoname(self) -> Optional[AlbumTagGeoname]: ...

class AlbumTagGeoname:
    @property
    def id(self) -> int: ...
    @property
    def name(self) -> str: ...
    @property
    def full_name(self) -> str: ...

class AlbumTrack:
    @property
    def id(self) -> int: ...
    @property
    def title(self) -> str: ...
    @property
    def track_number(self) -> Optional[int]:
        """Track number (one indexed), is None for tracks"""

    @property
    def duration(self) -> Optional[float]:
        """Duration is undefined for unstreamable tracks or unreleased tracks"""

    @property
    def streaming_url(self) -> dict[str, str]: ...
    @property
    def band_id(self) -> int: ...
    @property
    def band_name(self) -> str: ...
    @property
    def label(self) -> Optional[str]: ...
    @property
    def label_id(self) -> Optional[int]: ...
    @property
    def encodings_id(self) -> Optional[int]: ...
    @property
    def album_id(self) -> Optional[int]: ...
    @property
    def album_title(self) -> Optional[str]: ...
    @property
    def is_streamable(self) -> bool: ...
    @property
    def has_lyrics(self) -> bool: ...
    @property
    def image(self) -> AlbumImage: ...
    @property
    def purchase_options(self) -> PurchaseOptions: ...

class PurchaseOptions:
    @property
    def is_set_price(self) -> bool: ...
    @property
    def price(self) -> Optional[float]: ...
    @property
    def currency(self) -> Optional[str]: ...
    @property
    def is_purchasable(self) -> bool: ...
    @property
    def has_digital_download(self) -> bool: ...
    @property
    def require_email(self) -> bool:
        """
        Require an email address for a free download, see
        https://get.bandcamp.help/hc/en-us/articles/23020667057943-How-do-I-collect-emails
        """

class Artist:
    @property
    def id(self) -> int: ...
    @property
    def name(self) -> str: ...
    @property
    def image(self) -> Image: ...
    @property
    def bio(self) -> Optional[str]: ...
    @property
    def url(self) -> str: ...
    @property
    def sites(self) -> list[ArtistSite]: ...
    @property
    def location(self) -> Optional[str]: ...
    @property
    def discography(self) -> list[ArtistDiscographyEntry]: ...
    @property
    def artists(self) -> list[LabelArtist]:
        """artists are only present for labels"""

class ArtistSite:
    @property
    def title(self) -> str: ...
    @property
    def url(self) -> str: ...

class ArtistDiscographyEntry:
    @property
    def id(self) -> int: ...
    @property
    def item_type(self) -> ArtistDiscographyEntryType: ...
    @property
    def artist_name(self) -> Optional[str]:
        """Artist name of this specific release"""

    @property
    def band_id(self) -> int:
        """Name of the band this is released under"""

    @property
    def band_name(self) -> str: ...
    @property
    def title(self) -> str: ...
    @property
    def image(self) -> AlbumImage: ...
    @property
    def is_purchasable(self) -> bool: ...
    @property
    def release_date(self) -> datetime: ...

class LabelArtist:
    """Artist entry for a label"""

    @property
    def id(self) -> int: ...
    @property
    def name(self) -> str: ...
    @property
    def image(self) -> Image: ...
    @property
    def location(self) -> Optional[str]: ...

class SearchResultItemArtist:
    @property
    def artist_id(self) -> int: ...
    @property
    def image(self) -> Image: ...
    @property
    def name(self) -> str: ...
    @property
    def location(self) -> Optional[str]: ...
    @property
    def is_label(self) -> bool: ...
    @property
    def tags(self) -> list[str]: ...
    @property
    def genre(self) -> Optional[str]: ...
    @property
    def url(self) -> str: ...

class BandcampUrl:
    @property
    def artist_url(self) -> str: ...
    @property
    def item_url(self) -> str: ...

class SearchResultItemAlbum:
    @property
    def album_id(self) -> int: ...
    @property
    def image(self) -> AlbumImage: ...
    @property
    def name(self) -> str: ...
    @property
    def band_id(self) -> int: ...
    @property
    def band_name(self) -> str: ...
    @property
    def url(self) -> BandcampUrl: ...

class SearchResultItemTrack:
    @property
    def track_id(self) -> int: ...
    @property
    def image(self) -> AlbumImage: ...
    @property
    def name(self) -> str: ...
    @property
    def band_id(self) -> int: ...
    @property
    def band_name(self) -> str: ...
    @property
    def album_id(self) -> Optional[int]: ...
    @property
    def album_name(self) -> Optional[str]: ...
    @property
    def url(self) -> BandcampUrl: ...

class SearchResultItemFan:
    @property
    def fan_id(self) -> int: ...
    @property
    def image(self) -> Image: ...
    @property
    def name(self) -> str: ...
    @property
    def username(self) -> str: ...
    @property
    def collection_size(self) -> int: ...
    @property
    def genre_name(self) -> str: ...
    @property
    def url(self) -> str: ...

class ImageResolution:
    Full = 0
    Px25 = 22
    Px50 = 42
    Px60x45 = 140
    Px40x80 = 161
    Px90 = 101
    Px60x100 = 160
    Px100 = 3
    Px100x75 = 29
    Px120 = 21
    Px124 = 8
    Px135 = 15
    Px138 = 12
    Px140 = 50
    Px144x108 = 38
    Px150 = 7
    Px168x126 = 37
    Px172 = 11
    Px180 = 206
    Px200 = 44
    Px210 = 9
    Px210Gift = 165
    """Image with "Gift" in the top right hand corner"""

    Px240 = 205
    Px270 = 70
    Px280 = 201
    Px300 = 4
    Px350 = 2
    Px350Grayscale = 300
    """Grayscale 350x350px image"""

    Px360 = 204
    Px368 = 14
    Px368x276 = 33
    Px380 = 13
    Px380x285 = 32
    Px400x300 = 36
    Px420 = 200
    Px422 = 170
    Px540 = 71
    Px640x124 = 120
    Px646 = 171
    Px700 = 5
    Px715x402 = 27
    Px768x432 = 28
    Px800x600 = 26
    Px900x468 = 220
    Px975x180PNG = 100
    Px1024 = 20
    Px1024PNG = 31
    Px1200 = 10
    Px1280x720 = 150
    Px720x1280 = 151
    Px3000PNG = 1
