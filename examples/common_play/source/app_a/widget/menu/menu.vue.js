_$pi.define("app_a/widget/menu/menu.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./menu.vue.tpl", "pi_gui/widget/direct", "pi_gui/engine/animation_tools", "pi_gui/engine/tools"], function (require, exports, module, direct_1, menu_vue_tpl_1, direct_2, animation_tools_1, tools_1) {
    "use strict";

    exports.initMeta = void 0;
    class Menu {
        constructor() {
            this.selected = true;
            this.showAnim = true;
            this.image = "";
            this.canUse = null;
        }
        attach() {
            const parent = direct_2.getDomNode(this);
            parent.childNodes[0].childNodes.map(node => node.setAttribute('hsi', this.canUse && this.canUse() ? [0, 0, 1] : [0, -80, 0]));
        }
        afterUpdate() {
            if (this.selected) {
                this.showAnim = true;
                const node = direct_2.getDomNode(direct_2.findElementByAttr(this, 'id', this.image));
                const init_anim_cmd = new tools_1.AnimationCmd('openAnim');
                init_anim_cmd.iteration = 1;
                init_anim_cmd.duration = 500;
                init_anim_cmd.fillMode = 'both';
                const keyFrames = {
                    name: 'openAnim',
                    attrs: [{
                        key: animation_tools_1.FrameClassKeys.transform,
                        data: [[0, tools_1.Tools.readTransform(tools_1.Tools.analy('transform', 'scale(1,1)'))], [1, tools_1.Tools.readTransform(tools_1.Tools.analy('transform', 'scale(1.1,1.2)'))]]
                    }]
                };
                const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(init_anim_cmd, keyFrames);
                // 启动动画
                node.style.addAnimation(runtimeAnimation);
            } else {
                this.showAnim = false;
                const node = direct_2.getDomNode(direct_2.findElementByAttr(this, 'id', this.image));
                const init_anim_cmd = new tools_1.AnimationCmd('openAnim');
                init_anim_cmd.iteration = 1;
                init_anim_cmd.duration = 250;
                init_anim_cmd.fillMode = 'both';
                const keyFrames = {
                    name: 'openAnim',
                    attrs: [{
                        key: animation_tools_1.FrameClassKeys.transform,
                        data: [[0, tools_1.Tools.readTransform(tools_1.Tools.analy('transform', 'scale(1.05,1.15)'))], [1, tools_1.Tools.readTransform(tools_1.Tools.analy('transform', 'scale(1)'))]]
                    }]
                };
                const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(init_anim_cmd, keyFrames);
                // 启动动画
                node.style.addAnimation(runtimeAnimation);
            }
            const parent = direct_2.getDomNode(this);
            parent.childNodes[0].childNodes.map(node => node.setAttribute('hsi', this.canUse && this.canUse() ? [0, 0, 1] : [0, -80, 0]));
        }
    }
    exports.default = Menu;
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/menu/menu.vue.tpl.ts",
            _$cssPath = "app_a/widget/menu/menu.vue.wcss",
            _$cssHash = 1505823009;
        Menu["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: menu_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Menu, ["selected", "image", "name"]);
    direct_1.addField(Menu, ['showAnim', 'canUse']);
});