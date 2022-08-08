_$pi.define("app_b/feat/list_item.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./list_item.vue.tpl", "pi_utils/util/logger"], function (require, exports, module, direct_1, list_item_vue_tpl_1, logger_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    class ListItem {
        constructor() {
            this.handleClick = () => {
                let { func, name } = this.info;
                logD(name, 'clicked!');
                func && func();
            };
        }
    }
    exports.default = ListItem;
    exports.initMeta = () => {
        let _$tpl = "app_b/feat/list_item.vue.tpl.ts",
            _$cssPath = "app_b/feat/list_item.vue.wcss",
            _$cssHash = 1316494607;
        ListItem["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: list_item_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(ListItem, ["info"]);
    direct_1.addField(ListItem, ['handleClick']);
});