export declare class Meme {
  get key(): string;
  get info(): MemeInfo;
  generate(
    images: Array<Image>,
    texts: Array<string>,
    options: Record<string, OptionValue>
  ): MemeResult;
  generatePreview(
    options?: Record<string, OptionValue> | undefined | null
  ): MemeResult;
}

export declare function getVersion(): string;

export declare function getMeme(key: string): Meme | null;

export const enum MemeSortBy {
  Key = 0,
  Keywords = 1,
  KeywordsPinyin = 2,
  DateCreated = 3,
  DateModified = 4
}

export declare function getMemeKeys(
  sortBy?: MemeSortBy | undefined | null,
  sortReverse?: boolean | undefined | null
): Array<string>;

export declare function getMemes(
  sortBy?: MemeSortBy | undefined | null,
  sortReverse?: boolean | undefined | null
): Array<Meme>;

export declare function searchMemes(
  query: string,
  includeTags?: boolean | undefined | null
): Array<string>;

export interface MemeInfo {
  key: string;
  params: MemeParams;
  keywords: Array<string>;
  shortcuts: Array<MemeShortcut>;
  tags: Set<string>;
  dateCreated: Date;
  dateModified: Date;
}

export interface MemeParams {
  minImages: number;
  maxImages: number;
  minTexts: number;
  maxTexts: number;
  defaultTexts: Array<string>;
  options: Array<MemeOption>;
}

export interface MemeShortcut {
  pattern: string;
  humanized?: string;
  names: Array<string>;
  texts: Array<string>;
  options: Record<string, OptionValue>;
}

export type MemeOption =
  | { type: "Boolean"; field0: BooleanOption }
  | { type: "String"; field0: StringOption }
  | { type: "Integer"; field0: IntegerOption }
  | { type: "Float"; field0: FloatOption };

export interface BooleanOption {
  name: string;
  default?: boolean;
  description?: string;
  parserFlags: ParserFlags;
}

export interface StringOption {
  name: string;
  default?: string;
  choices?: Array<string>;
  description?: string;
  parserFlags: ParserFlags;
}

export interface IntegerOption {
  name: string;
  default?: number;
  minimum?: number;
  maximum?: number;
  description?: string;
  parserFlags: ParserFlags;
}

export interface FloatOption {
  name: string;
  default?: number;
  minimum?: number;
  maximum?: number;
  description?: string;
  parserFlags: ParserFlags;
}

export type OptionValue =
  | { type: "Boolean"; field0: boolean }
  | { type: "String"; field0: string }
  | { type: "Integer"; field0: number }
  | { type: "Float"; field0: number };

export interface ParserFlags {
  short: boolean;
  long: boolean;
  shortAliases: Array<string>;
  longAliases: Array<string>;
}

export interface Image {
  name: string;
  data: Buffer;
}

export type MemeResult =
  | { type: "Ok"; field0: Buffer }
  | { type: "Err"; field0: Error };

export type Error =
  | { type: "ImageDecodeError"; field0: ImageDecodeError }
  | { type: "ImageEncodeError"; field0: ImageEncodeError }
  | { type: "ImageAssetMissing"; field0: ImageAssetMissing }
  | { type: "DeserializeError"; field0: DeserializeError }
  | { type: "ImageNumberMismatch"; field0: ImageNumberMismatch }
  | { type: "TextNumberMismatch"; field0: TextNumberMismatch }
  | { type: "TextOverLength"; field0: TextOverLength }
  | { type: "MemeFeedback"; field0: MemeFeedback };

export interface ImageDecodeError {
  error: string;
}

export interface ImageEncodeError {
  error: string;
}

export interface ImageAssetMissing {
  path: string;
}

export interface DeserializeError {
  error: string;
}

export interface ImageNumberMismatch {
  min: number;
  max: number;
  actual: number;
}

export interface TextNumberMismatch {
  min: number;
  max: number;
  actual: number;
}

export interface TextOverLength {
  text: string;
}

export interface MemeFeedback {
  feedback: string;
}

export namespace Resources {
  export function checkResources(): void;

  export function checkResourcesInBackground(): void;
}

export namespace Tools {
  export function renderMemeList(
    renderMemeListParams: RenderMemeListParams
  ): ImageResult;

  export function renderMemeStatistics(
    renderMemeStatisticsParams: RenderMemeStatisticsParams
  ): ImageResult;

  export interface RenderMemeListParams {
    memeProperties?: Record<string, MemeProperties>;
    excludeMemes?: Array<string>;
    sortBy?: MemeSortBy;
    sortReverse?: boolean;
    textTemplate?: string;
    addCategoryIcon?: boolean;
  }

  export interface MemeProperties {
    disabled?: boolean;
    hot?: boolean;
    new?: boolean;
  }

  export interface RenderMemeStatisticsParams {
    title: string;
    statisticsType: MemeStatisticsType;
    data: Array<[string, number]>;
  }

  export const enum MemeStatisticsType {
    MemeCount = 0,
    TimeCount = 1,
  }

  export type ImageResult =
    | { type: "Ok"; field0: Buffer }
    | { type: "Err"; field0: Error };

  export type ImagesResult =
    | { type: "Ok"; field0: Array<Buffer> }
    | { type: "Err"; field0: Error };

  export namespace ImageOperations {
    export function inspect(image: Buffer): ImageInfoResult;

    export function flipHorizontal(image: Buffer): ImageResult;

    export function flipVertical(image: Buffer): ImageResult;

    export function rotate(image: Buffer, options: RotateOptions): ImageResult;

    export function resize(image: Buffer, options: ResizeOptions): ImageResult;

    export function crop(image: Buffer, options: CropOptions): ImageResult;

    export function grayscale(image: Buffer): ImageResult;

    export function invert(image: Buffer): ImageResult;

    export function mergeHorizontal(images: Array<Buffer>): ImageResult;

    export function mergeVertical(images: Array<Buffer>): ImageResult;

    export function gifSplit(image: Buffer): ImagesResult;

    export function gifMerge(
      images: Array<Buffer>,
      options: GifMergeOptions
    ): ImageResult;

    export function gifReverse(image: Buffer): ImageResult;

    export function gifChangeDuration(
      image: Buffer,
      options: GifChangeDurationOptions
    ): ImageResult;

    export type ImageInfoResult =
      | { type: "Ok"; field0: ImageInfo }
      | { type: "Err"; field0: Error };

    export interface ImageInfo {
      width: number;
      height: number;
      isMultiFrame: boolean;
      frameCount?: number;
      averageDuration?: number;
    }

    export interface RotateOptions {
      degrees?: number;
    }

    export interface ResizeOptions {
      width?: number;
      height?: number;
    }

    export interface CropOptions {
      left?: number;
      top?: number;
      right?: number;
      bottom?: number;
    }

    export interface GifMergeOptions {
      duration?: number;
    }

    export interface GifChangeDurationOptions {
      duration: number;
    }
  }
}
