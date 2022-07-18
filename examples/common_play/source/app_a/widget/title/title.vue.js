_$pi.define("app_a/widget/title/title.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./title.vue.tpl"], function (require, exports, module, direct_1, title_vue_tpl_1) {
    "use strict";

    exports.initMeta = void 0;
    class Title {
        constructor() {
            this.title = '';
        }
    }
    exports.default = Title;
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/title/title.vue.tpl.ts",
            _$cssPath = "app_a/widget/title/title.vue.wcss",
            _$cssHash = 2810977962;
        Title["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: title_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Title, ["title"]);
});