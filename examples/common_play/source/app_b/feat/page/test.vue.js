_$pi.define("app_b/feat/page/test.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./test.vue.tpl", "pi_common/ui/main_root", "app_a/widget/tips/tips.vue", "app_a/widget/pop/pop.vue", "app_a/widget/dialog/dialog.vue", "app_b/widget/open_treasure/open_treasure.vue"], function (require, exports, module, direct_1, test_vue_tpl_1, main_root_1, tips_vue_1, pop_vue_1, dialog_vue_1, open_treasure_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    class Test {
        constructor() {
            this.number = 10;
            this.nextTime = Date.now() + 10 * 1000;
            this.popUp = () => {
                main_root_1.pop(dialog_vue_1.default, { title: '测试', widget: pop_vue_1.default });
            };
        }
        increase() {
            this.number += 1000;
        }
        setTimer() {
            this.nextTime = Date.now() + 60 * 1000;
        }
        openTreasure() {
            main_root_1.pop(open_treasure_vue_1.default, { boxType: 2, rest: 6 });
        }
        timeEnd() {
            tips_vue_1.showTips('timeEnd!!!');
        }
        closePage() {
            main_root_1.close(this);
        }
    }
    exports.default = Test;
    exports.initMeta = () => {
        let _$tpl = "app_b/feat/page/test.vue.tpl.ts",
            _$cssPath = "app_b/feat/page/test.vue.wcss",
            _$cssHash = 1497705234;
        Test["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: test_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Test, ["number", "nextTime"]);
    direct_1.addField(Test, ['popUp']);
});