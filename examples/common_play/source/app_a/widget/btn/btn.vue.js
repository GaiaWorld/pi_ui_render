_$pi.define("app_a/widget/btn/btn.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./btn.vue.tpl", "pi_common/ui/utils", "pi_gui/widget/direct"], function (require, exports, module, direct_1, btn_vue_tpl_1, utils_1, direct_2) {
    "use strict";

    exports.initMeta = void 0;
    class Btn {
        constructor() {
            this.type = '';
            this.metaImg = "";
            this.weight = 500;
            this.spacing = 6;
            this.fontSize = 36;
            this.fontColor = '#ffffff';
            this.strokeSize = 4;
            this.strokeColor = '#ad5e28';
            this.shoadowColor = '#B56B27';
            this.text = '';
            this.clickBtn = () => {
                direct_2.emit(this, 'ev-click', {});
            };
            this.down = () => {
                utils_1.playScaleAnimation(direct_2.getDomNode(this), 1, 0.8);
            };
            this.up = () => {
                utils_1.playScaleAnimation(direct_2.getDomNode(this), 0.8, 1);
            };
        }
    }
    exports.default = Btn;
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/btn/btn.vue.tpl.ts",
            _$cssPath = "app_a/widget/btn/btn.vue.wcss",
            _$cssHash = 224311753;
        Btn["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: btn_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Btn, ["type", "metaImg", "shoadowColor", "strokeSize", "strokeColor", "fontSize", "fontColor", "spacing", "weight", "text"]);
    direct_1.addField(Btn, ['clickBtn', 'down', 'up']);
});