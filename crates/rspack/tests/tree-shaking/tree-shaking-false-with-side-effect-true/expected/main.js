(self['webpackChunkwebpack'] = self['webpackChunkwebpack'] || []).push([["main"], {
"./ b.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "b", {
    enumerable: true,
    get: ()=>b
});
const b = 3;
},
"./a.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "a", {
    enumerable: true,
    get: ()=>a
});
const a = 3;
},
"./index.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
const _aJs = __webpack_require__("./a.js");
const _bJs = __webpack_require__("./ b.js");
_aJs.a;
_bJs.b;
},

},function(__webpack_require__) {
__webpack_require__("./index.js");
}
]);