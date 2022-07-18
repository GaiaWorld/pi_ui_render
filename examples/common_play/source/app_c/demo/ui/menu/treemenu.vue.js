_$pi.define("app_c/demo/ui/menu/treemenu.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./treemenu.vue.tpl", "pi_gui/widget/direct"], function (require, exports, module, direct_1, treemenu_vue_tpl_1, direct_2) {
    "use strict";

    exports.initMeta = void 0;
    // ============================== 导出
    /**
     * @description 导出组件Widget类
     * @example
     */
    class TreeMenu {
        click(_e) {
            if (this.tree.show.leaf) {
                direct_2.emit(this, "ev-open", this.tree.cmd); // 触发打开事件
            } else if (this.tree.arr) {
                this.tree.show.select = !this.tree.show.select;
                this.tree = this.tree; // 触发重绘
            }
            let handler = this.btnWidget.getHandler();
            if (handler) {
                handler.setData("selectSid", this.tree.show.sid);
            }
        }
    }
    exports.default = TreeMenu;
    // ============================== 本地
    // ============================== 立即执行
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/menu/treemenu.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/menu/treemenu.vue.wcss",
            _$cssHash = 4011702308;
        TreeMenu["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: treemenu_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(TreeMenu, ["tree", "btnWidget"]);
});