from ... import ImageDecodeError, ImageEncodeError

class ImageInfo:
    width: int
    height: int
    is_multi_frame: bool
    frame_count: int | None
    average_duration: float | None

def inspect(
    image: bytes,
) -> ImageInfo | ImageDecodeError: ...
def flip_horizontal(
    image: bytes,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def flip_vertical(
    image: bytes,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def rotate(
    image: bytes,
    degrees: float | None = 90.0,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def resize(
    image: bytes,
    width: int | None = None,
    height: int | None = None,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def crop(
    image: bytes,
    left: int | None = None,
    top: int | None = None,
    right: int | None = None,
    bottom: int | None = None,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def grayscale(
    image: bytes,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def invert(
    image: bytes,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def merge_horizontal(
    images: list[bytes],
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def merge_vertical(
    images: list[bytes],
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def gif_split(
    image: bytes,
) -> list[bytes] | ImageDecodeError | ImageEncodeError: ...
def gif_merge(
    images: list[bytes],
    duration: float | None = 0.1,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def gif_reverse(
    image: bytes,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
def gif_change_duration(
    image: bytes,
    duration: float,
) -> bytes | ImageDecodeError | ImageEncodeError: ...
