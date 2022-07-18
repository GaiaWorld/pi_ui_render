_$pi.define("app_c/demo/demo.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./demo.vue.tpl", "pi_utils/util/logger", "pi_common/ui/main_root", "./demo_single.vue"], function (require, exports, module, direct_1, demo_vue_tpl_1, logger_1, main_root_1, demo_single_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    class Demo {
        constructor() {
            this.cfglist = [];
            this.quit = () => {
                main_root_1.close(this);
            };
            this.click = i => {
                const cfg = this.cfglist[i - 0];
                if (cfg) {
                    if (Array.isArray(cfg[1])) {
                        main_root_1.open(demo_single_vue_1.default, { list: cfg[1] });
                    } else {
                        let func = cfg[1];
                        func && func();
                    }
                }
            };
        }
    }
    exports.default = Demo;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/demo.vue.tpl.ts",
            _$cssPath = "app_c/demo/demo.vue.wcss",
            _$cssHash = 731494153;
        Demo["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: demo_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Demo, ["cfglist"]);
    direct_1.addField(Demo, ['quit', 'click']);
});