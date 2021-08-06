/* tslint:disable */
/* eslint-disable */
/**
*/
export function main(): void;
/**
*/
export class CounterApp {
  free(): void;
/**
* @param {Function} update_view_fn
* @returns {CounterApp}
*/
  static new(update_view_fn: Function): CounterApp;
/**
* @param {string} action
*/
  dispatch(action: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_counterapp_free: (a: number) => void;
  readonly counterapp_new: (a: number) => number;
  readonly counterapp_dispatch: (a: number, b: number, c: number) => void;
  readonly main: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
