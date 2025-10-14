from __future__ import annotations

import asyncio
from .bandcamp_lib import *


async def _fetch_artist_sync(artist_id: int) -> Artist:
    return await fetch_artist(artist_id)


def fetch_artist_sync(artist_id: int) -> Artist:
    return asyncio.run(_fetch_artist_sync(artist_id))


async def _fetch_album_sync(artist_id: int, album_id: int) -> Album:
    return await fetch_album(artist_id, album_id)


def fetch_album_sync(artist_id: int, album_id: int) -> Album:
    return asyncio.run(_fetch_album_sync(artist_id, album_id))


async def _fetch_track_sync(artist_id: int, track_id: int) -> Album:
    return await fetch_track(artist_id, track_id)


def fetch_track_sync(artist_id: int, track_id: int) -> Album:
    return asyncio.run(_fetch_track_sync(artist_id, track_id))


async def _search_sync(
    query: str,
) -> list[
    SearchResultItemAlbum
    | SearchResultItemArtist
    | SearchResultItemTrack
    | SearchResultItemFan
]:
    return asyncio.run(search(query))


def search_sync(
    query: str,
) -> list[
    SearchResultItemAlbum
    | SearchResultItemArtist
    | SearchResultItemTrack
    | SearchResultItemFan
]:
    return asyncio.run(_search_sync(query))


async def _album_from_url_sync(url: str) -> Album:
    return await album_from_url(url)


def album_from_url_sync(url: str) -> Album:
    return asyncio.run(_album_from_url_sync(url))


async def _artist_from_url_sync(url: str) -> Artist:
    return await artist_from_url(url)


def artist_from_url_sync(url: str) -> Artist:
    return asyncio.run(_artist_from_url_sync(url))


async def _track_from_url_sync(url: str) -> Album:
    return await track_from_url(url)


def track_from_url_sync(url: str) -> Album:
    return asyncio.run(_track_from_url_sync(url))


__doc__ = bandcamp_lib.__doc__
__all__ = [
    "fetch_artist_sync",
    "fetch_album_sync",
    "fetch_track_sync",
    "search_sync",
    "album_from_url_sync",
    "artist_from_url_sync",
    "track_from_url_sync",
]
if hasattr(bandcamp_lib, "__all__"):
    __all__ += bandcamp_lib.__all__
