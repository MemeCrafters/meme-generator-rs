import type { MemeInfo, UploadImageResponse, ImageResponse, ErrorResponse } from './types'

const BASE = '/api'

export class MemeError extends Error {
  code: number
  data: any

  constructor(code: number, message: string, data: any = null) {
    super(message)
    this.name = 'MemeError'
    this.code = code
    this.data = data
  }
}

async function request<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${url}`, options)
  if (!res.ok) {
    const body = await res.json().catch(() => null)
    if (body && 'code' in body) {
      const err = body as ErrorResponse
      throw new MemeError(err.code, err.message, err.data)
    }
    throw new MemeError(0, `HTTP ${res.status}: ${res.statusText}`)
  }
  return res.json()
}

export async function getMemeKeys(): Promise<string[]> {
  return request<string[]>('/meme/keys')
}

export async function getMemeInfos(): Promise<MemeInfo[]> {
  return request<MemeInfo[]>('/meme/infos')
}

export async function getMemeInfo(key: string): Promise<MemeInfo> {
  return request<MemeInfo>(`/memes/${key}/info`)
}

export async function searchMemes(query: string, includeTags = false): Promise<string[]> {
  const params = new URLSearchParams({ query })
  if (includeTags) params.set('include_tags', 'true')
  return request<string[]>(`/meme/search?${params}`)
}

export async function getMemePreview(key: string): Promise<ImageResponse> {
  return request<ImageResponse>(`/memes/${key}/preview`)
}

export async function uploadImage(file: File): Promise<UploadImageResponse> {
  const formData = new FormData()
  formData.append('file', file)
  const res = await fetch(`${BASE}/image/upload/multipart`, {
    method: 'POST',
    body: formData,
  })
  if (!res.ok) {
    const body = await res.json().catch(() => null)
    throw new Error(body?.message || `Upload failed: ${res.statusText}`)
  }
  return res.json()
}

export async function generateMeme(
  key: string,
  images: { name: string; id: string }[],
  texts: string[],
  options: Record<string, any>
): Promise<ImageResponse> {
  return request<ImageResponse>(`/memes/${key}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ images, texts, options }),
  })
}

export function getImageUrl(imageId: string): string {
  return `${BASE}/image/${imageId}`
}

export async function getVersion(): Promise<string> {
  const res = await fetch(`${BASE}/meme/version`)
  return res.text()
}
