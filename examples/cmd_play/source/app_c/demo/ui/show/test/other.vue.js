_$pi.define("app_c/demo/ui/show/test/other.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./other.vue.tpl"], function (require, exports, module, direct_1, other_vue_tpl_1) {
    "use strict";

    exports.initMeta = exports.Text = void 0;
    class TestWidget {
        constructor() {
            this.count = 1;
        }
        change() {
            this.count += 3;
        }
        firstPaint() {
            console.log(11111111);
        }
    }
    exports.default = TestWidget;
    class Text {}
    exports.Text = Text;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/test/other.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/test/other.vue.wcss",
            _$cssHash = 4152699387;
        Text["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: other_vue_tpl_1.BW6, cssHash: _$cssHash };
        TestWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: other_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(TestWidget, ["count"]);
    direct_1.defineAccessors(Text, ["text"]);
});