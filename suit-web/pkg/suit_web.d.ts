/* tslint:disable */
/* eslint-disable */
/**
*/
export function main(): void;
/**
*/
export function init_panic_hook(): void;
/**
*/
export class CounterApp {
  free(): void;
/**
* @param {WebClient} client
* @returns {CounterApp}
*/
  static new(client: WebClient): CounterApp;
/**
*/
  increment(): void;
/**
*/
  decrement(): void;
}
/**
*/
export class WebClient {
  free(): void;
/**
* @param {any} options
* @returns {WebClient}
*/
  static new(options: any): WebClient;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_counterapp_free: (a: number) => void;
  readonly counterapp_new: (a: number) => number;
  readonly counterapp_increment: (a: number) => void;
  readonly counterapp_decrement: (a: number) => void;
  readonly main: () => void;
  readonly init_panic_hook: () => void;
  readonly __wbg_webclient_free: (a: number) => void;
  readonly webclient_new: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
