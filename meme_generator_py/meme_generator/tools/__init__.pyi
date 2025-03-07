from enum import Enum
from typing import Union

from .. import ImageEncodeError

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

def render_meme_list(
    meme_properties: dict[str, MemeProperties] = {},
    exclude_memes: list[str] = [],
    sort_by: MemeSortBy = MemeSortBy.KeywordsPinyin,
    sort_reverse: bool = False,
    text_template: str = "{index}. {keywords}",
    add_category_icon: bool = True,
) -> Union[bytes, ImageEncodeError]: ...
def render_meme_statistics(
    title: str,
    statistics_type: MemeStatisticsType,
    data: list[tuple[str, int]],
) -> Union[bytes, ImageEncodeError]: ...
