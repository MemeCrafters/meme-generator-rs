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

export declare function getMemeKeys(): Array<string>;

export declare function getMemes(): Array<Meme>;

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
  export declare function checkResources(): void;

  export declare function checkResourcesInBackground(): void;
}

export namespace Tools {
  export declare function renderMemeList(
    renderMemeListParams: RenderMemeListParams
  ): ImageResult;

  export declare function renderMemeStatistics(
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

  export declare const enum MemeSortBy {
    Key = 0,
    Keywords = 1,
    KeywordsPinyin = 2,
    DateCreated = 3,
    DateModified = 4,
  }

  export interface RenderMemeStatisticsParams {
    title: string;
    statisticsType: MemeStatisticsType;
    data: Array<[string, number]>;
  }

  export declare const enum MemeStatisticsType {
    MemeCount = 0,
    TimeCount = 1,
  }

  export namespace ImageOperations {
    export declare function inspect(image: Buffer): ImageInfoResult;

    export declare function flipHorizontal(image: Buffer): ImageResult;

    export declare function flipVertical(image: Buffer): ImageResult;

    export declare function rotate(
      image: Buffer,
      options: RotateOptions
    ): ImageResult;

    export declare function resize(
      image: Buffer,
      options: ResizeOptions
    ): ImageResult;

    export declare function crop(
      image: Buffer,
      options: CropOptions
    ): ImageResult;

    export declare function grayscale(image: Buffer): ImageResult;

    export declare function invert(image: Buffer): ImageResult;

    export declare function mergeHorizontal(images: Array<Buffer>): ImageResult;

    export declare function mergeVertical(images: Array<Buffer>): ImageResult;

    export declare function gifSplit(image: Buffer): ImagesResult;

    export declare function gifMerge(
      images: Array<Buffer>,
      options: GifMergeOptions
    ): ImageResult;

    export declare function gifReverse(image: Buffer): ImageResult;

    export declare function gifChangeDuration(
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

    export type ImageResult =
      | { type: "Ok"; field0: Buffer }
      | { type: "Err"; field0: Error };

    export type ImagesResult =
      | { type: "Ok"; field0: Array<Buffer> }
      | { type: "Err"; field0: Error };

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
