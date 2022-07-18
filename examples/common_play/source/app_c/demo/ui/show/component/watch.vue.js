_$pi.define("app_c/demo/ui/show/component/watch.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./watch.vue.tpl", "pi_gui/widget/direct"], function (require, exports, module, direct_1, watch_vue_tpl_1, direct_2) {
    "use strict";

    exports.initMeta = void 0;
    class Watch extends direct_2.WidgetBase {
        constructor() {
            super(...arguments);
            this.a1 = 0;
            this.a2 = 0;
            this.a3 = 0;
            this.a4 = 0;
            this.a5 = 0;
        }
        modifyA1() {
            this.a1 = Math.ceil(Math.random() * 100);
        }
        modifyA2() {
            this.a2 = Math.ceil(Math.random() * 100);
        }
        // 监听a1的改变(watch注解没有参数时，自动分析代码中使用的变量作为监听属性)
        watchA1() {
            this.a3 = this.a1;
        }
        // 监听a1的改变(watch注解没有参数时，自动分析代码中使用的变量作为监听属性)
        watchA1A2() {
            this.a4 = Math.ceil(Math.random() * 100);
            this.a5 = Math.ceil(Math.random() * 100);
        }
    }
    exports.default = Watch;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/component/watch.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/component/watch.vue.wcss",
            _$cssHash = 1624158360;
        Watch["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: watch_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.addWatch(Watch, "watchA1", function (info) {
        return info.dirty0 & 1;
    });
    direct_1.addWatch(Watch, "watchA1A2", function (info) {
        return info.dirty0 & 3;
    });
    direct_1.defineAccessors(Watch, ["a1", "a2", "a3", "a4", "a5"]);
});