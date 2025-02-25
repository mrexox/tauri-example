// We need to declare the `window` object to avoid TypeScript errors. But instead you
// can use the `@tauri-apps/api/core` module for the same thing.
//
// The difference is that if you distribute the app to the web too, the `@tauri-apps/api/core`
// seems to be an extra dependency that you don't need.
interface Window {
  __TAURI__: {
    core: {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      invoke: (_f: string, _args?: any) => Promise<any>
      convertFileSrc: (_f: string) => string
    }
  }
}

declare const window: Window

export const isTauri = typeof window !== 'undefined' && !!window.__TAURI__

interface GoogleAuthOptions {
  authCode: string
  redirectUri: string
}

export async function getGoogleAuthCode(): Promise<GoogleAuthOptions | Record<string, never>> {
  if (!isTauri) return {}

  const { invoke } = window.__TAURI__.core

  const [authCode, redirectUri] = await invoke('google_auth_code')

  return { authCode, redirectUri }
}

export async function sidecarSend(event: string, data: any): Promise<void> {
  if (!isTauri) return

  const { invoke } = window.__TAURI__.core

  // Build a message
  // {
  //   event: 'event_name',
  //   data: any JSON data
  // }
  const message = JSON.stringify({ event, data })

  console.log('sending message')
  await invoke('sidecar_send', { message })
}

export async function firstImage(subdir: string): Promise<string | null> {
  if (!isTauri) return null

  const { invoke } = window.__TAURI__.core

  return await invoke('first_image_path', { subdir })
}

export function localAssetUrl(path: string): string | null {
  return window.__TAURI__.core.convertFileSrc(path)
}
