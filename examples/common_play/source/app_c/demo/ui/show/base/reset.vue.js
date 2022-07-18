_$pi.define("app_c/demo/ui/show/base/reset.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./reset.vue.tpl"], function (require, exports, module, direct_1, reset_vue_tpl_1) {
    "use strict";

    exports.initMeta = void 0;
    class ResetWidget {
        constructor() {
            this.width = '100px';
            this.left = undefined;
            this.right = '100px';
        }
        tap(feild) {
            this[feild] = this[feild] ? undefined : "100px";
        }
    }
    exports.default = ResetWidget;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/base/reset.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/base/reset.vue.wcss",
            _$cssHash = 3964075548;
        ResetWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: reset_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(ResetWidget, ["width", "left", "right"]);
});