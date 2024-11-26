interface ImportMetaEnv {
  readonly VITE_BACKEND_PATH: string;
  readonly VITE_GOOGLE_MAP_API_KEY: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}