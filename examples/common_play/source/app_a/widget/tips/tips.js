_$pi.define("app_a/widget/tips/tips", ["require", "exports", "module", "pi_gui/widget/forelet", "pi_gui/widget/widget", "pi_common/ui/main_root"], function (require, exports, module, forelet_1, widget_1, main_root_1) {
    "use strict";

    exports.TipsPlugin = exports.Tips = exports.forelet = void 0;
    exports.forelet = new forelet_1.Forelet();
    class Tips extends widget_1.Widget {
        constructor() {
            super(...arguments);
            this.animEnd = () => {
                this.ok && this.ok();
            };
        }
    }
    exports.Tips = Tips;
    /**
     * 挂在到Widget原型上，组件内部可使用 this.$message('hello') 形式进行调用
     * 不进行插件安装则和正常ui组件使用无差别
     *  */
    exports.TipsPlugin = {
        install() {
            widget_1.Widget.prototype.$message = text => {
                main_root_1.openTip('app_a-widget-tips-tips', { text });
            };
        }
    };
});