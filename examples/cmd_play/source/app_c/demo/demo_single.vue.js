_$pi.define("app_c/demo/demo_single.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./demo_single.vue.tpl", "pi_common/ui/main_root"], function (require, exports, module, direct_1, demo_single_vue_tpl_1, main_root_1) {
    "use strict";

    exports.initMeta = void 0;
    class DemoSingle {
        constructor() {
            this.msg = "";
            this.list = [];
        }
        click(i) {
            var _a;
            const func = this.list[i][1];
            if (func) {
                (_a = func()) === null || _a === void 0 ? void 0 : _a.then(msg => {
                    this.msg = msg;
                }).catch(e => {
                    this.msg = e;
                });
            }
        }
        quit() {
            main_root_1.close(this);
        }
    }
    exports.default = DemoSingle;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/demo_single.vue.tpl.ts",
            _$cssPath = "app_c/demo/demo_single.vue.wcss",
            _$cssHash = 716257371;
        DemoSingle["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: demo_single_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(DemoSingle, ["list", "msg"]);
});