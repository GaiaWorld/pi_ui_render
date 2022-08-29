_$pi.define("app_c/demo/ui/show/base/input.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./input.vue.tpl", "pi_gui/widget/direct"], function (require, exports, module, direct_1, input_vue_tpl_1, direct_2) {
    "use strict";

    exports.initMeta = void 0;
    class InputWidget extends direct_2.WidgetBase {
        constructor() {
            super(...arguments);
            this.value1 = "";
            this.value2 = "";
            this.value3 = "";
        }
        changeCall(e) {
            this.value1 = e.current.value;
            console.log("input change call", e, this.value1);
        }
        changeCall2(e) {
            this.value2 = e.current.value;
            console.log("input change call", e, this.value2);
        }
        changeCall3(e) {
            this.value3 = e.current.value;
            console.log("input change call", e, this.value3);
        }
        inputChange(e) {
            // console.log("================== inputChange ",e);
        }
    }
    exports.default = InputWidget;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/base/input.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/base/input.vue.wcss",
            _$cssHash = 3637456325;
        InputWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: input_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(InputWidget, ["value1", "value2", "value3"]);
});