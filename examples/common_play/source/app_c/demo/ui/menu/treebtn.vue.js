_$pi.define("app_c/demo/ui/menu/treebtn.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./treebtn.vue.tpl", "pi_gui/widget/handler", "pi_common/ui/handler"], function (require, exports, module, direct_1, treebtn_vue_tpl_1, handler_1, handler_2) {
    "use strict";

    exports.initMeta = exports.forelet = void 0;
    exports.forelet = new handler_1.SimpleHandler();
    class TreeBtn extends handler_2.WithHandler {}
    exports.default = TreeBtn;
    TreeBtn.setHandler(exports.forelet);
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/menu/treebtn.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/menu/treebtn.vue.wcss",
            _$cssHash = 3806411067;
        TreeBtn["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: treebtn_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(TreeBtn, ["selectSid", "sid", "leaf", "select", "text"]);
});