from datetime import datetime

class ParserFlags:
    short: bool
    long: bool
    short_aliases: list[str]
    long_aliases: list[str]

class BooleanOption:
    name: str
    default: bool | None
    description: str | None
    parser_flags: ParserFlags

class StringOption:
    name: str
    default: str | None
    choices: list[str] | None
    description: str | None
    parser_flags: ParserFlags

class IntegerOption:
    name: str
    default: int | None
    minimum: int | None
    maximum: int | None
    description: str | None
    parser_flags: ParserFlags

class FloatOption:
    name: str
    default: float | None
    minimum: float | None
    maximum: float | None
    description: str | None
    parser_flags: ParserFlags

class MemeParams:
    min_images: int
    max_images: int
    min_texts: int
    max_texts: int
    default_texts: list[str]
    options: list[BooleanOption | StringOption | IntegerOption | FloatOption]

class MemeShortcut:
    pattern: str
    humanized: str | None
    names: list[str]
    texts: list[str]
    options: dict[str, bool | str | int | float]

class MemeInfo:
    key: str
    params: MemeParams
    keywords: list[str]
    shortcuts: list[MemeShortcut]
    tags: set[str]
    date_created: datetime
    date_modified: datetime

class Image:
    def __new__(cls, name: str, data: bytes): ...

class ImageDecodeError:
    error: str

class ImageEncodeError:
    error: str

class ImageAssetMissing:
    path: str

class DeserializeError:
    error: str

class ImageNumberMismatch:
    min: int
    max: int
    actual: int

class TextNumberMismatch:
    min: int
    max: int
    actual: int

class TextOverLength:
    text: str

class MemeFeedback:
    feedback: str

class Meme:
    @property
    def key(self) -> str: ...
    @property
    def info(self) -> MemeInfo: ...
    def generate(
        self,
        images: list[Image],
        texts: list[str],
        options: dict[str, bool | str | int | float],
    ) -> (
        bytes
        | ImageDecodeError
        | ImageEncodeError
        | ImageAssetMissing
        | DeserializeError
        | ImageNumberMismatch
        | TextNumberMismatch
        | TextOverLength
        | MemeFeedback
    ): ...
    def generate_preview(
        self,
        options: dict[str, bool | str | int | float] = {},
    ) -> (
        bytes
        | ImageEncodeError
        | ImageAssetMissing
        | DeserializeError
        | TextOverLength
        | MemeFeedback
    ): ...

def get_version() -> str: ...
def get_meme(key: str) -> Meme: ...
def get_memes() -> list[Meme]: ...
def get_meme_keys() -> list[str]: ...
def search_memes(query: str, include_tags: bool = False) -> list[str]: ...
