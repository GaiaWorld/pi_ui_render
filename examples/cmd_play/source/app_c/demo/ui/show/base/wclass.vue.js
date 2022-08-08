_$pi.define("app_c/demo/ui/show/base/wclass.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./wclass.vue.tpl", "pi_gui/widget/direct", "pi_common/ui/element_tools"], function (require, exports, module, direct_1, wclass_vue_tpl_1, direct_2, element_tools_1) {
    "use strict";

    exports.initMeta = void 0;
    class ResetWidget extends direct_2.WidgetBase {
        constructor() {
            super(...arguments);
            this.list = [1, 2, 3, 4, 6];
            this.removed = false;
            this.curLang = 'zh';
        }
        tap(e, i) {
            console.log(e, i);
            const target = e.current;
            // moveElement2topLayer(target);
            if (this.curLang === 'zh') {
                this.curLang = 'en';
            } else {
                this.curLang = 'zh';
            }
            element_tools_1.switchLang(this.curLang);
        }
    }
    exports.default = ResetWidget;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/base/wclass.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/base/wclass.vue.wcss",
            _$cssHash = 3236063441;
        ResetWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: wclass_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(ResetWidget, ["list"]);
    direct_1.addField(ResetWidget, ['removed', 'parentNode', 'nextNode', 'curLang']);
});