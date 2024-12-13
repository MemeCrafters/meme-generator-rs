from dataclasses import dataclass

from .meme_generator import BooleanOption as BooleanOption
from .meme_generator import DeserializeError as DeserializeError
from .meme_generator import FloatOption as FloatOption
from .meme_generator import ImageDecodeError as ImageDecodeError
from .meme_generator import ImageEncodeError as ImageEncodeError
from .meme_generator import ImageNumberMismatch as ImageNumberMismatch
from .meme_generator import IntegerOption as IntegerOption
from .meme_generator import IOError as IOError
from .meme_generator import Meme as Meme
from .meme_generator import MemeFeedback as MemeFeedback
from .meme_generator import MemeInfo as MemeInfo
from .meme_generator import MemeParams as MemeParams
from .meme_generator import MemeShortcut as MemeShortcut
from .meme_generator import ParserFlags as ParserFlags
from .meme_generator import StringOption as StringOption
from .meme_generator import TextNumberMismatch as TextNumberMismatch
from .meme_generator import TextOverLength as TextOverLength
from .meme_generator import check_resources as check_resources
from .meme_generator import get_meme as get_meme
from .meme_generator import get_meme_keys as get_meme_keys
from .meme_generator import get_memes as get_memes


@dataclass
class RawImage:
    name: str
    data: bytes
