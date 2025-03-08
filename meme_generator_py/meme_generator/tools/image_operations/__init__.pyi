from typing import Optional, Union

from ... import ImageDecodeError, ImageEncodeError

class ImageInfo:
    width: int
    height: int
    is_multi_frame: bool
    frame_count: Optional[int]
    average_duration: Optional[float]

def inspect(
    image: bytes,
) -> Union[ImageInfo, ImageDecodeError]: ...
def flip_horizontal(
    image: bytes,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def flip_vertical(
    image: bytes,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def rotate(
    image: bytes,
    degrees: Optional[float] = 90.0,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def resize(
    image: bytes,
    width: Optional[int] = None,
    height: Optional[int] = None,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def crop(
    image: bytes,
    left: Optional[int] = None,
    top: Optional[int] = None,
    right: Optional[int] = None,
    bottom: Optional[int] = None,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def grayscale(
    image: bytes,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def invert(
    image: bytes,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def merge_horizontal(
    images: list[bytes],
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def merge_vertical(
    images: list[bytes],
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def gif_split(
    image: bytes,
) -> Union[list[bytes], ImageDecodeError, ImageEncodeError]: ...
def gif_merge(
    images: list[bytes],
    duration: Optional[float] = 0.1,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def gif_reverse(
    image: bytes,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
def gif_change_duration(
    image: bytes,
    duration: float,
) -> Union[bytes, ImageDecodeError, ImageEncodeError]: ...
