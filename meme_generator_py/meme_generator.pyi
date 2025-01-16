from datetime import datetime
from enum import Enum
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
    options: dict[str, OptionValue]

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

OptionValue = Union[bool, str, int, float]

MemeError = Union[
    ImageDecodeError,
    ImageEncodeError,
    ImageAssetMissing,
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
        images: list[Image],
        text: list[str],
        options: dict[str, OptionValue],
    ) -> MemeResult: ...
    def generate_preview(self) -> MemeResult: ...

class MemeProperties:
    def __new__(cls, disabled: bool = False, hot: bool = False, new: bool = False): ...

class MemeSortBy(Enum):
    Key = 0
    Keywords = 1
    KeywordsPinyin = 2
    DateCreated = 3
    DateModified = 4

class MemeStatisticsType(Enum):
    MemeCount = 0
    TimeCount = 1

def get_version() -> str: ...
def get_meme(key: str) -> Meme: ...
def get_memes() -> list[Meme]: ...
def get_meme_keys() -> list[str]: ...
def search_memes(query: str, include_tags: bool = False) -> list[str]: ...
def check_resources() -> None: ...
def check_resources_in_background() -> None: ...
def render_meme_list(
    meme_properties: dict[str, MemeProperties] = {},
    exclude_memes: list[str] = [],
    sort_by: MemeSortBy = MemeSortBy.KeywordsPinyin,
    sort_reverse: bool = False,
    text_template: str = "{index}. {keywords}",
    add_category_icon: bool = True,
) -> MemeResult: ...
def render_meme_statistics(
    title: str,
    statistics_type: MemeStatisticsType,
    data: list[tuple[str, int]],
) -> MemeResult: ...
