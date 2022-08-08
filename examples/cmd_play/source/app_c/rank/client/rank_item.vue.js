_$pi.define("app_c/rank/client/rank_item.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./rank_item.vue.tpl"], function (require, exports, module, direct_1, rank_item_vue_tpl_1) {
    "use strict";

    exports.initMeta = void 0;
    class RankItem {
        constructor() {
            this.i = 0;
            this.v = {
                uid: "",
                num: 0
            };
        }
    }
    exports.default = RankItem;
    exports.initMeta = () => {
        let _$tpl = "app_c/rank/client/rank_item.vue.tpl.ts",
            _$cssPath = "app_c/rank/client/rank_item.vue.wcss",
            _$cssHash = 3856664783;
        RankItem["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: rank_item_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(RankItem, ["i", "v"]);
});