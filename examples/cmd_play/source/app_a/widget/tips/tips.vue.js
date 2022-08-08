_$pi.define("app_a/widget/tips/tips.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./tips.vue.tpl", "pi_common/ui/main_root"], function (require, exports, module, direct_1, tips_vue_tpl_1, main_root_1) {
    "use strict";

    exports.initMeta = exports.showTips = void 0;
    class Tips {
        constructor() {
            this.text = "";
            this.animEnd = () => {
                setTimeout(() => {
                    this.ok && this.ok();
                }, 2000);
            };
        }
    }
    exports.default = Tips;
    exports.showTips = text => {
        return main_root_1.pop(Tips, { text: text }, 'pop_tip');
    };
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/tips/tips.vue.tpl.ts",
            _$cssPath = "app_a/widget/tips/tips.vue.wcss",
            _$cssHash = 3333665390;
        Tips["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: tips_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Tips, ["text"]);
    direct_1.addField(Tips, ['ok', 'animEnd']);
});