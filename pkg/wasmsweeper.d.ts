/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function getBoardState(): string;
/**
* @param {number} x
* @param {number} y
*/
export function openField(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function toggleFlag(x: number, y: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly getBoardState: (a: number) => void;
  readonly openField: (a: number, b: number) => void;
  readonly toggleFlag: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
