// Auto-generated with deno_bindgen
import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.1/plug.ts"
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}
function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}
function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as Deno.UnsafePointer)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}
const opts = {
  name: "denqlite",
  url: (new URL("../target/debug", import.meta.url)).toString(),
  policy: CachePolicy.NONE,
}
const _lib = await prepare(opts, {
  close: { parameters: ["usize"], result: "usize", nonblocking: false },
  execute: {
    parameters: ["usize", "pointer", "usize", "pointer", "usize"],
    result: "usize",
    nonblocking: false,
  },
  fill_error: {
    parameters: ["usize", "pointer", "usize"],
    result: "void",
    nonblocking: false,
  },
  fill_result: {
    parameters: ["usize", "pointer", "usize"],
    result: "void",
    nonblocking: false,
  },
  get_error_length: {
    parameters: ["usize"],
    result: "usize",
    nonblocking: false,
  },
  get_result_length: {
    parameters: ["usize"],
    result: "usize",
    nonblocking: false,
  },
  open: {
    parameters: ["usize", "pointer", "usize"],
    result: "usize",
    nonblocking: false,
  },
  query: {
    parameters: ["usize", "pointer", "usize", "pointer", "usize", "usize"],
    result: "usize",
    nonblocking: false,
  },
})

export function close(a0: number) {
  let rawResult = _lib.symbols.close(a0)
  const result = rawResult
  return result
}
export function execute(a0: number, a1: Uint8Array, a2: Uint8Array) {
  const a1_buf = encode(a1)
  const a2_buf = encode(a2)
  let rawResult = _lib.symbols.execute(
    a0,
    a1_buf,
    a1_buf.byteLength,
    a2_buf,
    a2_buf.byteLength,
  )
  const result = rawResult
  return result
}
export function fill_error(a0: number, a1: Uint8Array) {
  const a1_buf = encode(a1)
  let rawResult = _lib.symbols.fill_error(a0, a1_buf, a1_buf.byteLength)
  const result = rawResult
  return result
}
export function fill_result(a0: number, a1: Uint8Array) {
  const a1_buf = encode(a1)
  let rawResult = _lib.symbols.fill_result(a0, a1_buf, a1_buf.byteLength)
  const result = rawResult
  return result
}
export function get_error_length(a0: number) {
  let rawResult = _lib.symbols.get_error_length(a0)
  const result = rawResult
  return result
}
export function get_result_length(a0: number) {
  let rawResult = _lib.symbols.get_result_length(a0)
  const result = rawResult
  return result
}
export function open(a0: number, a1: string) {
  const a1_buf = encode(a1)
  let rawResult = _lib.symbols.open(a0, a1_buf, a1_buf.byteLength)
  const result = rawResult
  return result
}
export function query(a0: number, a1: Uint8Array, a2: Uint8Array, a3: number) {
  const a1_buf = encode(a1)
  const a2_buf = encode(a2)
  let rawResult = _lib.symbols.query(
    a0,
    a1_buf,
    a1_buf.byteLength,
    a2_buf,
    a2_buf.byteLength,
    a3,
  )
  const result = rawResult
  return result
}
