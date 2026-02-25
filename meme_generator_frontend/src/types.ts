export interface ParserFlags {
  short: boolean
  long: boolean
  short_aliases: string[]
  long_aliases: string[]
}

export interface MemeOptionBoolean {
  type: 'boolean'
  name: string
  default: boolean | null
  description: string | null
  parser_flags: ParserFlags
}

export interface MemeOptionString {
  type: 'string'
  name: string
  default: string | null
  choices: string[] | null
  description: string | null
  parser_flags: ParserFlags
}

export interface MemeOptionInteger {
  type: 'integer'
  name: string
  default: number | null
  minimum: number | null
  maximum: number | null
  description: string | null
  parser_flags: ParserFlags
}

export interface MemeOptionFloat {
  type: 'float'
  name: string
  default: number | null
  minimum: number | null
  maximum: number | null
  description: string | null
  parser_flags: ParserFlags
}

export type MemeOption =
  | MemeOptionBoolean
  | MemeOptionString
  | MemeOptionInteger
  | MemeOptionFloat

export interface MemeParams {
  min_images: number
  max_images: number
  min_texts: number
  max_texts: number
  default_texts: string[]
  options: MemeOption[]
}

export interface MemeShortcut {
  pattern: string
  humanized: string | null
  names: string[]
  texts: string[]
  options: Record<string, any>
}

export interface MemeInfo {
  key: string
  params: MemeParams
  keywords: string[]
  shortcuts: MemeShortcut[]
  tags: string[]
  date_created: string
  date_modified: string
}

export interface UploadImageResponse {
  image_id: string
}

export interface ImageResponse {
  image_id: string
}

export interface ErrorResponse {
  code: number
  message: string
  data: any
}

export interface ImageItem {
  name: string
  id: string
  preview?: string // local object URL for preview
  file?: File
}
