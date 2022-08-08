_$pi.define("app_c/demo/ui/show/component/hover_cmd.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./hover_cmd.vue.tpl", "pi_gui/widget/direct", "pi_common/ui/hover_cmd"], function (require, exports, module, direct_1, hover_cmd_vue_tpl_1, direct_2, hover_cmd_1) {
    "use strict";

    exports.initMeta = void 0;
    // 指令在初始化时注册，一般只需要注册一次，注册后，所有的widget都可使用该指令。注意指令名称不要重复
    direct_2.directive("myhover", hover_cmd_1.hoverCmd);
    class HoverCmdTest {
        constructor() {
            this.emitHover1 = (event, args) => {
                this.tip1 = `emit ${args} ${event.source.uniqueID}`;
                console.log("emit", args, event);
            };
            this.cancelHover1 = (event, args) => {
                this.tip1 = `cancel ${args} ${event.source.uniqueID}`;
                console.log("cancel", args, event);
            };
            this.emitHover2 = (event, args) => {
                this.tip2 = `emit ${args} ${event.source.uniqueID}`;
                console.log("emit", args, event);
            };
            this.cancelHover2 = (event, args) => {
                this.tip2 = `cancel ${args} ${event.source.uniqueID}`;
                console.log("cancel", args, event);
            };
        }
    }
    exports.default = HoverCmdTest;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/component/hover_cmd.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/component/hover_cmd.vue.wcss",
            _$cssHash = 812041377;
        HoverCmdTest["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: hover_cmd_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(HoverCmdTest, ["emitHover1", "cancelHover1", "tip1", "emitHover2", "cancelHover2", "tip2"]);
});