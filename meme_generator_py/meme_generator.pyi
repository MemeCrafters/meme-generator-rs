from datetime import datetime
from typing import Optional, Union

class ParserFlags:
    short: bool
    long: bool
    short_aliases: list[str]
    long_aliases: list[str]

class BooleanOption:
    name: str
    default: Optional[bool]
    description: Optional[str]
    parser_flags: ParserFlags

class StringOption:
    name: str
    default: Optional[str]
    choices: Optional[list[str]]
    description: Optional[str]
    parser_flags: ParserFlags

class IntegerOption:
    name: str
    default: Optional[int]
    minimum: Optional[int]
    maximum: Optional[int]
    description: Optional[str]
    parser_flags: ParserFlags

class FloatOption:
    name: str
    default: Optional[float]
    minimum: Optional[float]
    maximum: Optional[float]
    description: Optional[str]
    parser_flags: ParserFlags

MemeOption = Union[BooleanOption, StringOption, IntegerOption, FloatOption]

class MemeParams:
    min_images: int
    max_images: int
    min_texts: int
    max_texts: int
    default_texts: list[str]
    options: list[MemeOption]

class MemeShortcut:
    pattern: str
    humanized: Optional[str]
    names: list[str]
    texts: list[str]
    parser_args: list[str]

class MemeInfo:
    key: str
    params: MemeParams
    keywords: list[str]
    shortcuts: list[MemeShortcut]
    tags: set[str]
    date_created: datetime
    date_modified: datetime

class ImageDecodeError:
    error: str

class ImageEncodeError:
    error: str

class IOError:
    error: str

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

OptionValue = Union[bool, str, int, float]

MemeError = Union[
    ImageDecodeError,
    ImageEncodeError,
    IOError,
    DeserializeError,
    ImageNumberMismatch,
    TextNumberMismatch,
    TextOverLength,
    MemeFeedback,
]

MemeResult = Union[bytes, MemeError]

class Meme:
    @property
    def key(self) -> str: ...
    @property
    def info(self) -> MemeInfo: ...
    def generate(
        self,
        images: list[tuple[str, bytes]],
        text: list[str],
        options: dict[str, OptionValue],
    ) -> MemeResult: ...
    def generate_preview(self) -> MemeResult: ...

def get_meme(key: str) -> Meme: ...
def get_memes() -> list[Meme]: ...
def get_meme_keys() -> list[str]: ...
def check_resources() -> None: ...
def check_resources_in_background() -> None: ...
