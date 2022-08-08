_$pi.define("app_c/demo/ui/show/base/opacity.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./opacity.vue.tpl"], function (require, exports, module, direct_1, opacity_vue_tpl_1) {
    "use strict";

    exports.initMeta = void 0;
    class OpacityWidget {
        constructor() {
            this.opacity = 1.0; // -180~180
        }
        change() {
            this.opacity -= 0.1;
            if (this.opacity < 0) {
                this.opacity = 1.0;
            }
        }
    }
    exports.default = OpacityWidget;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/base/opacity.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/base/opacity.vue.wcss",
            _$cssHash = 1653847692;
        OpacityWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: opacity_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(OpacityWidget, ["opacity"]);
});