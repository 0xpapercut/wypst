"use strict";
(self["webpackChunkwypst"] = self["webpackChunkwypst"] || []).push([["src_core_pkg_core_js"],{

/***/ "./src/core/pkg/core.js":
/*!******************************!*\
  !*** ./src/core/pkg/core.js ***!
  \******************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_f851667af71bcfc6),
/* harmony export */   __wbg_new_56693dbed0c32988: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_56693dbed0c32988),
/* harmony export */   __wbg_new_898a68150f225f2e: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_898a68150f225f2e),
/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_abda76e883ba8a5f),
/* harmony export */   __wbg_new_b51585de1b234aff: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_b51585de1b234aff),
/* harmony export */   __wbg_set_20cbc34131e76824: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_20cbc34131e76824),
/* harmony export */   __wbg_set_502d29070ea18557: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_502d29070ea18557),
/* harmony export */   __wbg_set_bedc3d02d0f05eb0: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_bedc3d02d0f05eb0),
/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),
/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_658279fe44541cf6),
/* harmony export */   __wbindgen_debug_string: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_debug_string),
/* harmony export */   __wbindgen_error_new: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_error_new),
/* harmony export */   __wbindgen_is_string: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_is_string),
/* harmony export */   __wbindgen_number_new: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_number_new),
/* harmony export */   __wbindgen_object_clone_ref: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_clone_ref),
/* harmony export */   __wbindgen_object_drop_ref: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),
/* harmony export */   __wbindgen_string_new: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_string_new),
/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw),
/* harmony export */   parse_tree: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.parse_tree),
/* harmony export */   typst_tree_str: () => (/* reexport safe */ _core_bg_js__WEBPACK_IMPORTED_MODULE_0__.typst_tree_str)
/* harmony export */ });
/* harmony import */ var _core_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./core_bg.wasm */ "./src/core/pkg/core_bg.wasm");
/* harmony import */ var _core_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./core_bg.js */ "./src/core/pkg/core_bg.js");
var __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_core_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);
_core_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];


(0,_core_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_core_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);

__webpack_async_result__();
} catch(e) { __webpack_async_result__(e); } });

/***/ }),

/***/ "./src/core/pkg/core_bg.js":
/*!*********************************!*\
  !*** ./src/core/pkg/core_bg.js ***!
  \*********************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* binding */ __wbg_error_f851667af71bcfc6),
/* harmony export */   __wbg_new_56693dbed0c32988: () => (/* binding */ __wbg_new_56693dbed0c32988),
/* harmony export */   __wbg_new_898a68150f225f2e: () => (/* binding */ __wbg_new_898a68150f225f2e),
/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* binding */ __wbg_new_abda76e883ba8a5f),
/* harmony export */   __wbg_new_b51585de1b234aff: () => (/* binding */ __wbg_new_b51585de1b234aff),
/* harmony export */   __wbg_set_20cbc34131e76824: () => (/* binding */ __wbg_set_20cbc34131e76824),
/* harmony export */   __wbg_set_502d29070ea18557: () => (/* binding */ __wbg_set_502d29070ea18557),
/* harmony export */   __wbg_set_bedc3d02d0f05eb0: () => (/* binding */ __wbg_set_bedc3d02d0f05eb0),
/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),
/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* binding */ __wbg_stack_658279fe44541cf6),
/* harmony export */   __wbindgen_debug_string: () => (/* binding */ __wbindgen_debug_string),
/* harmony export */   __wbindgen_error_new: () => (/* binding */ __wbindgen_error_new),
/* harmony export */   __wbindgen_is_string: () => (/* binding */ __wbindgen_is_string),
/* harmony export */   __wbindgen_number_new: () => (/* binding */ __wbindgen_number_new),
/* harmony export */   __wbindgen_object_clone_ref: () => (/* binding */ __wbindgen_object_clone_ref),
/* harmony export */   __wbindgen_object_drop_ref: () => (/* binding */ __wbindgen_object_drop_ref),
/* harmony export */   __wbindgen_string_new: () => (/* binding */ __wbindgen_string_new),
/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw),
/* harmony export */   parse_tree: () => (/* binding */ parse_tree),
/* harmony export */   typst_tree_str: () => (/* binding */ typst_tree_str)
/* harmony export */ });
/* module decorator */ module = __webpack_require__.hmd(module);
let wasm;
function __wbg_set_wasm(val) {
  wasm = val;
}
const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;
let cachedTextDecoder = new lTextDecoder('utf-8', {
  ignoreBOM: true,
  fatal: true
});
cachedTextDecoder.decode();
let cachedUint8Memory0 = null;
function getUint8Memory0() {
  if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
const heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);
let heap_next = heap.length;
function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  if (typeof heap_next !== 'number') throw new Error('corrupt heap');
  heap[idx] = obj;
  return idx;
}
function getObject(idx) {
  return heap[idx];
}
function dropObject(idx) {
  if (idx < 132) return;
  heap[idx] = heap_next;
  heap_next = idx;
}
function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
function _assertBoolean(n) {
  if (typeof n !== 'boolean') {
    throw new Error('expected a boolean argument');
  }
}
function debugString(val) {
  // primitive types
  const type = typeof val;
  if (type == 'number' || type == 'boolean' || val == null) {
    return `${val}`;
  }
  if (type == 'string') {
    return `"${val}"`;
  }
  if (type == 'symbol') {
    const description = val.description;
    if (description == null) {
      return 'Symbol';
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == 'function') {
    const name = val.name;
    if (typeof name == 'string' && name.length > 0) {
      return `Function(${name})`;
    } else {
      return 'Function';
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = '[';
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ', ' + debugString(val[i]);
    }
    debug += ']';
    return debug;
  }
  // Test for built-in
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    // Failed to match the standard '[object ClassName]'
    return toString.call(val);
  }
  if (className == 'Object') {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return 'Object(' + JSON.stringify(val) + ')';
    } catch (_) {
      return 'Object';
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  // TODO we could test for more things here, like `Set`s and `Map`s.
  return className;
}
let WASM_VECTOR_LEN = 0;
const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;
let cachedTextEncoder = new lTextEncoder('utf-8');
const encodeString = typeof cachedTextEncoder.encodeInto === 'function' ? function (arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function (arg, view) {
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  return {
    read: arg.length,
    written: buf.length
  };
};
function passStringToWasm0(arg, malloc, realloc) {
  if (typeof arg !== 'string') throw new Error('expected a string argument');
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length, 1) >>> 0;
    getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;
  const mem = getUint8Memory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    if (ret.read !== arg.length) throw new Error('failed to pass whole string');
    offset += ret.written;
  }
  WASM_VECTOR_LEN = offset;
  return ptr;
}
let cachedInt32Memory0 = null;
function getInt32Memory0() {
  if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}
/**
* @param {string} expression
* @returns {any}
*/
function parse_tree(expression) {
  const ptr0 = passStringToWasm0(expression, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.parse_tree(ptr0, len0);
  return takeObject(ret);
}

/**
* @param {string} text
* @returns {any}
*/
function typst_tree_str(text) {
  const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.typst_tree_str(ptr0, len0);
  return takeObject(ret);
}
function logError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    let error = function () {
      try {
        return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
      } catch (_) {
        return "<failed to stringify thrown value>";
      }
    }();
    console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
    throw e;
  }
}
function __wbindgen_error_new(arg0, arg1) {
  const ret = new Error(getStringFromWasm0(arg0, arg1));
  return addHeapObject(ret);
}
;
function __wbindgen_string_new(arg0, arg1) {
  const ret = getStringFromWasm0(arg0, arg1);
  return addHeapObject(ret);
}
;
function __wbindgen_object_drop_ref(arg0) {
  takeObject(arg0);
}
;
function __wbg_error_f851667af71bcfc6() {
  return logError(function (arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
      deferred0_0 = arg0;
      deferred0_1 = arg1;
      console.error(getStringFromWasm0(arg0, arg1));
    } finally {
      wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
  }, arguments);
}
;
function __wbg_new_abda76e883ba8a5f() {
  return logError(function () {
    const ret = new Error();
    return addHeapObject(ret);
  }, arguments);
}
;
function __wbg_stack_658279fe44541cf6() {
  return logError(function (arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  }, arguments);
}
;
function __wbindgen_number_new(arg0) {
  const ret = arg0;
  return addHeapObject(ret);
}
;
function __wbindgen_object_clone_ref(arg0) {
  const ret = getObject(arg0);
  return addHeapObject(ret);
}
;
function __wbg_set_20cbc34131e76824() {
  return logError(function (arg0, arg1, arg2) {
    getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
  }, arguments);
}
;
function __wbg_new_898a68150f225f2e() {
  return logError(function () {
    const ret = new Array();
    return addHeapObject(ret);
  }, arguments);
}
;
function __wbg_set_502d29070ea18557() {
  return logError(function (arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
  }, arguments);
}
;
function __wbg_new_56693dbed0c32988() {
  return logError(function () {
    const ret = new Map();
    return addHeapObject(ret);
  }, arguments);
}
;
function __wbg_set_bedc3d02d0f05eb0() {
  return logError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
  }, arguments);
}
;
function __wbg_new_b51585de1b234aff() {
  return logError(function () {
    const ret = new Object();
    return addHeapObject(ret);
  }, arguments);
}
;
function __wbindgen_is_string(arg0) {
  const ret = typeof getObject(arg0) === 'string';
  _assertBoolean(ret);
  return ret;
}
;
function __wbindgen_debug_string(arg0, arg1) {
  const ret = debugString(getObject(arg1));
  const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len1 = WASM_VECTOR_LEN;
  getInt32Memory0()[arg0 / 4 + 1] = len1;
  getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}
;
function __wbindgen_throw(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}
;

/***/ }),

/***/ "./src/core/pkg/core_bg.wasm":
/*!***********************************!*\
  !*** ./src/core/pkg/core_bg.wasm ***!
  \***********************************/
/***/ ((module, exports, __webpack_require__) => {

/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./core_bg.js */ "./src/core/pkg/core_bg.js");
module.exports = __webpack_require__.v(exports, module.id, "2006cf51a75e0b60f55b", {
	"./core_bg.js": {
		"__wbindgen_error_new": WEBPACK_IMPORTED_MODULE_0.__wbindgen_error_new,
		"__wbindgen_string_new": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_new,
		"__wbindgen_object_drop_ref": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,
		"__wbg_error_f851667af71bcfc6": WEBPACK_IMPORTED_MODULE_0.__wbg_error_f851667af71bcfc6,
		"__wbg_new_abda76e883ba8a5f": WEBPACK_IMPORTED_MODULE_0.__wbg_new_abda76e883ba8a5f,
		"__wbg_stack_658279fe44541cf6": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_658279fe44541cf6,
		"__wbindgen_number_new": WEBPACK_IMPORTED_MODULE_0.__wbindgen_number_new,
		"__wbindgen_object_clone_ref": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_clone_ref,
		"__wbg_set_20cbc34131e76824": WEBPACK_IMPORTED_MODULE_0.__wbg_set_20cbc34131e76824,
		"__wbg_new_898a68150f225f2e": WEBPACK_IMPORTED_MODULE_0.__wbg_new_898a68150f225f2e,
		"__wbg_set_502d29070ea18557": WEBPACK_IMPORTED_MODULE_0.__wbg_set_502d29070ea18557,
		"__wbg_new_56693dbed0c32988": WEBPACK_IMPORTED_MODULE_0.__wbg_new_56693dbed0c32988,
		"__wbg_set_bedc3d02d0f05eb0": WEBPACK_IMPORTED_MODULE_0.__wbg_set_bedc3d02d0f05eb0,
		"__wbg_new_b51585de1b234aff": WEBPACK_IMPORTED_MODULE_0.__wbg_new_b51585de1b234aff,
		"__wbindgen_is_string": WEBPACK_IMPORTED_MODULE_0.__wbindgen_is_string,
		"__wbindgen_debug_string": WEBPACK_IMPORTED_MODULE_0.__wbindgen_debug_string,
		"__wbindgen_throw": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw
	}
});

/***/ })

}]);
//# sourceMappingURL=src_core_pkg_core_js.e03d117c0988ed3fc1e8.js.map